use super::mesh;
use super::mesh::Vertex;
use super::texture;

pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(device: &wgpu::Device, output_tex_format: wgpu::TextureFormat) -> Self {
        let vert_shader_module =
            device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let frag_shader_module =
            device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[], // diffuse texture and uniform bind group layouts would go here
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vert_shader_module,
                entry_point: "main",
                buffers: &[mesh::MeshVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: output_tex_format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None, // Add a `depth_format: Option<wgpu::TextureFormat>` param if this is needed.
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Self { pipeline }
    }

    /// Execute the `render_pipeline`, writing output to the `output_texture`. Then
    /// copy the texture to the `output_buffer`.
    pub fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mesh: &mesh::Mesh,
        screenshot_width: u32,
        screenshot_height: u32,
        output_texture: &texture::Texture,
        output_buffer: &wgpu::Buffer,
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
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            // render_pass.draw(0..3, 0..1);
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
}
