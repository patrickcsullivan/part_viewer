pub struct Texture<'a> {
    pub desc: wgpu::TextureDescriptor<'a>,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl<'a> Texture<'a> {
    /// Creates a texture onto which an image can be rendered.
    pub fn create_rgba_output_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        label: &'a str,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth: 1,
        };
        let desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::COPY_SRC | wgpu::TextureUsage::RENDER_ATTACHMENT,
            label: Some(label),
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            desc,
            texture,
            view,
        }
    }
}
