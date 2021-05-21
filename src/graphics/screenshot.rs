use super::texture;

pub struct ScreenshotDescriptor<'a> {
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

/// Create a render pipeline over the specified shaders.
fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    vs_src: wgpu::ShaderModuleDescriptor,
    fs_src: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let vs_module = device.create_shader_module(&vs_src);
    let fs_module = device.create_shader_module(&fs_src);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vs_module,
            entry_point: "main",
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: color_format,
                alpha_blend: wgpu::BlendState::REPLACE,
                color_blend: wgpu::BlendState::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
        },
        depth_stencil: None, // Add a `depth_format: Option<wgpu::TextureFormat>` param if this is needed.
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    })
}

/// Execute the `render_pipeline`, writing output to the `output_texture`. Then
/// copy the texture to the `output_buffer`.
fn render(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    screenshot_width: u32,
    screenshot_height: u32,
    output_texture: &texture::Texture,
    output_buffer: &wgpu::Buffer,
    render_pipeline: &wgpu::RenderPipeline,
) {
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &output_texture.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 0.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        };
        let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

        render_pass.set_pipeline(render_pipeline);
        render_pass.draw(0..3, 0..1);
    }

    let u32_size = std::mem::size_of::<u32>() as u32;
    encoder.copy_texture_to_buffer(
        wgpu::TextureCopyView {
            texture: &output_texture.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::BufferCopyView {
            buffer: &output_buffer,
            layout: wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: u32_size * screenshot_width,
                rows_per_image: screenshot_height,
            },
        },
        output_texture.desc.size,
    );

    queue.submit(Some(encoder.finish()));
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

pub async fn run<'a>(screenshot_desc: ScreenshotDescriptor<'a>) {
    let (device, queue) = request_device().await;

    let output_texture = texture::Texture::create_rgba_output_texture(
        &device,
        screenshot_desc.width,
        screenshot_desc.height,
        "Output Texture",
    );
    let output_buffer =
        create_output_buffer(&device, screenshot_desc.width, screenshot_desc.height);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let render_pipeline = create_render_pipeline(
        &device,
        &render_pipeline_layout,
        output_texture.desc.format,
        &[], // TODO: ADD VERTICES
        wgpu::include_spirv!("shader.vert.spv"),
        wgpu::include_spirv!("shader.frag.spv"),
    );
    render(
        &device,
        &queue,
        screenshot_desc.width,
        screenshot_desc.height,
        &output_texture,
        &output_buffer,
        &render_pipeline,
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
