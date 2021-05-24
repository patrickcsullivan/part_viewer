use super::camera;
use super::light;
use super::mesh;
use super::render_pipeline;
use super::texture;
use super::transformation;
use cgmath::Rotation3;

pub struct ScreenshotDescriptor<'a> {
    pub mesh_path: &'a str,
    pub dst_path: &'a str,
    pub width: u32,
    pub height: u32,
}

/// Request the GPU device and its queue.
async fn request_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        })
        .await
        .unwrap();
    adapter
        .request_device(&Default::default(), None)
        .await
        .unwrap()
}

/// Create the buffer onto which the output image will be written.
fn create_output_buffer(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Buffer {
    let u32_size = std::mem::size_of::<u32>() as u32;
    let output_buffer_size = (u32_size * width * height) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsage::COPY_DST
            // this tells wpgu that we want to read this buffer from the cpu
            | wgpu::BufferUsage::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    device.create_buffer(&output_buffer_desc)
}

/// Poll data from the device and write the output buffer to the destination path.
async fn save_buffer_to_image(
    device: &wgpu::Device,
    output_buffer: &wgpu::Buffer,
    dst_path: &str,
    width: u32,
    height: u32,
) {
    let buffer_slice = output_buffer.slice(..);

    // We have to create the mapping THEN device.poll() before await the
    // future. Otherwise the application will freeze.
    let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
    device.poll(wgpu::Maintain::Wait);
    mapping.await.unwrap();

    let data = buffer_slice.get_mapped_range();

    use image::{ImageBuffer, Rgba};
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, data).unwrap();
    buffer.save(dst_path).unwrap();
}

/// Generate a screenshot.
pub async fn run(screenshot_desc: ScreenshotDescriptor<'_>) {
    let (device, queue) = request_device().await;

    let depth_texture = texture::Texture::create_depth_texture(
        &device,
        screenshot_desc.width,
        screenshot_desc.height,
        "Depth Texture",
    );

    // TODO: Create model transformation from input data.
    let model_transformation = transformation::Transformation::new(
        &device,
        cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_x(), cgmath::Deg(-90.0)),
        cgmath::Vector3 {
            x: 0.0,
            y: -5.0,
            z: 0.0,
        },
    );

    // TODO: Create camera from input data.
    let camera = camera::Camera::new_perspective_camera(
        &device,
        (8.0, 0.0, 25.0),
        (0.0, 0.0, 0.0),
        screenshot_desc.width as f32 / screenshot_desc.height as f32,
        cgmath::Deg(45.0),
        0.1,
        100.0,
    );

    // TODO: Create light from input data.
    let point_light = light::PointLight::new(&device, (8.0, 12.0, 15.0), (1.0, 0.0, 1.0));

    let output_texture = texture::Texture::create_rgba_output_texture(
        &device,
        screenshot_desc.width,
        screenshot_desc.height,
        "Output Texture",
    );
    let output_buffer =
        create_output_buffer(&device, screenshot_desc.width, screenshot_desc.height);
    let render_pipeline = render_pipeline::RenderPipeline::new(
        &device,
        &model_transformation.bind_group_layout,
        &camera.bind_group_layout,
        &point_light.bind_group_layout,
        depth_texture.desc.format,
        output_texture.desc.format,
    );

    let mesh = mesh::Mesh::load(&device, screenshot_desc.mesh_path).unwrap();
    render_pipeline.render(
        &device,
        &queue,
        &mesh,
        &model_transformation.bind_group,
        &camera.bind_group,
        &point_light.bind_group,
        screenshot_desc.width,
        screenshot_desc.height,
        &depth_texture,
        &output_texture,
        &output_buffer,
    );
    save_buffer_to_image(
        &device,
        &output_buffer,
        screenshot_desc.dst_path,
        screenshot_desc.width,
        screenshot_desc.height,
    )
    .await;
    output_buffer.unmap();
}
