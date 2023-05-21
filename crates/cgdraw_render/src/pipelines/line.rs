pub struct LinePipeline {
    pub render_pipeline: wgpu::RenderPipeline,
}
impl LinePipeline {
    pub fn new(
        device: &wgpu::Device,
        topology: wgpu::PrimitiveTopology,
        index_format: wgpu::IndexFormat,
        format: wgpu::TextureFormat,
    ) -> Self {
        use std::borrow::Cow;

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("line_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shader_source = wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
            "../../../../assets/shaders/line.wgsl"
        )));

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Line shader"),
            source: shader_source,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                strip_index_format: Some(index_format),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self { render_pipeline }
    }
}
