pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let descriptor = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            /**
             * Especifica a largura, altura e profundidade da textura.
             */
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            /**
             * Especifica o número de níveis mipmap da textura.
             * Um nível de mipmap é uma versão reduzida da textura original usada
             * em distâncias maiores ou menores.
             */
            mip_level_count: 1,
            /**
             * Especifica o número de amostras de textura por pixel.
             */
            sample_count: 1,
            /**
             * Especifica a dimensão da textura.
             */
            dimension: wgpu::TextureDimension::D2,
            /**
             * Especifica o formato da textura.
             */
            format: Self::DEPTH_FORMAT,
            /**
             * Especifica o uso da textura.
             * Neste caso, a textura será usada como um anexo de renderização ou
             * como uma textura de ligação.
             */
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            /**
             * Especifica os formatos de visualização da textura.
             */
            view_formats: &[Self::DEPTH_FORMAT],
        };

        let texture = device.create_texture(&descriptor);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            /**
             * Especifica o modo de endereçamento da textura
             * no eixo U (horizontal).
             */
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            /**
             * Especifica o modo de endereçamento da textura
             * no eixo V (vertical).
             */
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            /**
             * Especifica o modo de endereçamento da textura
             * no eixo W (profundidade).
             */
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            /**
             * Especifica o modo de filtragem da textura
             * quando a textura é ampliada.
             */
            mag_filter: wgpu::FilterMode::Linear,
            /**
             * Especifica o modo de filtragem da textura
             * quando a textura é reduzida.
             */
            min_filter: wgpu::FilterMode::Linear,
            /**
             * Especifica o modo de filtragem da textura
             * quando a textura é reduzida ou ampliada.
             */
            mipmap_filter: wgpu::FilterMode::Nearest,
            /**
             * Especifica o limite inferior para o nível de detalhe (LOD) da textura.
             */
            compare: Some(wgpu::CompareFunction::LessEqual),
            /**
             * Especifica o limite inferior para o nível de detalhe (LOD) da textura.
             */
            lod_min_clamp: 0.0,
            /**
             * Especifica o limite superior para o nível de detalhe (LOD) da textura.
             */
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}
