use std::borrow::Cow;

use cgdraw_core::graphic::{Texture, Vertex};

pub struct MainPipeline {
    pub pipeline_layout: wgpu::PipelineLayout,
    pub pipeline: wgpu::RenderPipeline,
}

impl MainPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Self {
        let shader_source = wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
            "../../../assets/shaders/shader.wgsl"
        )));

        let shader_module_desc = wgpu::ShaderModuleDescriptor {
            label: Some("Main Shader"),
            source: shader_source,
        };

        let shader_module = device.create_shader_module(shader_module_desc);

        let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
            label: Some("Main Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        };

        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

        let vertex_state = wgpu::VertexState {
            module: &shader_module,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        };

        let fragment_state = wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        };

        let pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: Some("Main Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        };

        let pipeline = device.create_render_pipeline(&pipeline_desc);

        Self {
            pipeline,
            pipeline_layout,
        }
    }
}
