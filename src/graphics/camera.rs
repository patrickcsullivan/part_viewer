use cgmath::{Matrix4, Point3, Rad, Vector3};
use wgpu::util::DeviceExt;

/// Uniform data that can be sent to the shaders. Contains the camera view
/// projection matrix.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewProjectionUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl ViewProjectionUniform {
    /// Creates a new view projection matrix uniform, initialized to the
    /// identity matrix.
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    /// Set the uniform to the view projection matrix of the given camera.
    fn update_view_proj(&mut self, camera: &PerspectiveCameraDescrip) {
        self.view_proj = camera.view_projection_matrix().into();
    }
}

// The coordinate system in wgpu is based on DirectX and Metal's coordinate
// systems. That means that in normalized device coordinates, the x axis and y
// axis range from -1.0 to +1.0, and the z axis ranges from 0.0 to +1.0. The
// cgmath crate (like most game math crates) is built for OpenGL's coordinate
// system. This matrix will scale and translate our scene from OpenGL's
// coordinate sytem to WGPU's.
#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

struct PerspectiveCameraDescrip {
    position: Point3<f32>,
    target: Point3<f32>,
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl PerspectiveCameraDescrip {
    fn new<P: Into<Point3<f32>>, A: Into<Rad<f32>>>(
        position: P,
        target: P,
        aspect: f32,
        fovy: A,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            position: position.into(),
            target: target.into(),
            aspect,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    /// Returns the camera's inverse transformation matrix.
    ///
    /// When rendering a scene, rather than moving the camera, we keep the
    /// camera fixed at (0, 0, 1), and we use the camera's inverse
    /// transformation matrix to move all triangles in the world so that it
    /// appears as though the camera moved and the world remained still.
    fn inverse_transformation_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.target, Vector3::unit_y())
    }

    fn projection_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }

    fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.inverse_transformation_matrix()
    }
}

pub struct Camera {
    // TODO: uniform and ViewProjectionUniform could probably be private
    pub uniform: ViewProjectionUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new_perspective_camera<P: Into<Point3<f32>>, A: Into<Rad<f32>>>(
        device: &wgpu::Device,
        position: P,
        target: P,
        aspect: f32,
        fovy: A,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let descrip = PerspectiveCameraDescrip::new(position, target, aspect, fovy, znear, zfar);

        let mut uniform = ViewProjectionUniform::new();
        uniform.update_view_proj(&descrip);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Perspective Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Perspective Camera Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Perspective Camera Bind Group"),
        });

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
