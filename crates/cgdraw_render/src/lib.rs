use std::iter::once;

mod pipeline;

use cgdraw_core::graphic::Texture;
pub use pipeline::*;

pub struct Render<'a> {
    surface: &'a wgpu::Surface,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    main_pipeline: Option<MainPipeline>,
    default_view: Option<wgpu::TextureView>,
    depth_view: Option<Texture>,
}

impl<'a> Render<'a> {
    pub fn new(
        surface: &'a wgpu::Surface,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
    ) -> Self {
        Self {
            surface,
            device,
            queue,
            main_pipeline: None,
            default_view: None,
            depth_view: None,
        }
    }
}

impl<'a> Render<'a> {
    fn render_pass(self) {
        if self.default_view.is_some() && self.depth_view.is_some() && self.main_pipeline.is_some()
        {
            let depth_view = self.depth_view.unwrap().view;
            let default_view = self.default_view.unwrap();
            let main_pipeline = self.main_pipeline.unwrap().pipeline;

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

            let desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &default_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            };

            {
                let mut pass = encoder.begin_render_pass(&desc);

                pass.set_pipeline(&main_pipeline);
                pass.set_stencil_reference(0);
            }

            self.queue.submit(once(encoder.finish()));
        }
    }
}

impl<'a> Render<'a> {
    pub fn build(mut self, config: &'a wgpu::SurfaceConfiguration) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                self.surface.configure(self.device, config);
                self.surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        };

        self.default_view = Some(
            frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        self.depth_view = Some(Texture::create_depth_texture(self.device, config));

        self.main_pipeline = Some(MainPipeline::new(self.device, config.format));

        self.render_pass();

        frame.present();
    }
}
