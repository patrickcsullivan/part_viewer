use anyhow::*;
use wgpu::util::DeviceExt;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    position: [f32; 3],
    normal: [f32; 3],
}

impl Vertex for MeshVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Mesh {
    pub fn load(device: &wgpu::Device, mesh: &nom_stl::Mesh) -> Result<Self> {
        let mut vertices = Vec::new();
        for triangle in mesh.triangles() {
            // TODO: Do I need to make sure thse are CCW around normal?
            vertices.push(MeshVertex {
                position: triangle.vertices()[0],
                normal: triangle.normal(),
            });
            vertices.push(MeshVertex {
                position: triangle.vertices()[1],
                normal: triangle.normal(),
            });
            vertices.push(MeshVertex {
                position: triangle.vertices()[2],
                normal: triangle.normal(),
            });
        }

        let num_elements = vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&(0..num_elements).collect::<Vec<u32>>()),
            usage: wgpu::BufferUsage::INDEX,
        });

        Ok(Self {
            name: "Mesh".to_string(),
            vertex_buffer,
            index_buffer,
            num_indices: num_elements,
        })
    }
}
