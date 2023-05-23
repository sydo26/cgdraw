use std::iter::once;

mod graphic;
mod pipeline;
mod state;

use cgdraw_core::graphic::Texture;
use cgdraw_state::State;
pub use graphic::*;
pub use pipeline::*;
pub use state::*;

pub struct Render<'a> {
    pub state: &'a State,
    pub render_state: RenderState,
    default_view: Option<wgpu::TextureView>,
}

impl<'a> Render<'a> {
    pub fn new(state: &'a State, render_state: RenderState) -> Self {
        Self {
            state,
            render_state,
            default_view: None,
        }
    }
}

impl<'a> Render<'a> {
    fn render_pass(self) {
        if let Some(default_view) = self.default_view {
            let depth_view =
                Texture::create_depth_texture(&self.state.device, &self.state.surface_config).view;
            let main_pipeline =
                MainPipeline::new(&self.state.device, self.state.surface_config.format);

            let mut encoder =
                self.state
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

                pass.set_pipeline(&main_pipeline.pipeline);
                pass.set_stencil_reference(0);

                for vb in self.render_state.buffers.vertices.iter() {
                    pass.draw_vertices(vb);
                }
            }

            self.state.queue.submit(once(encoder.finish()));
        }
    }
}

impl<'a> Render<'a> {
    pub fn build(mut self) {
        let frame = match self.state.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                self.state
                    .surface
                    .configure(&self.state.device, &self.state.surface_config);
                self.state
                    .surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        };

        self.default_view = Some(
            frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        self.render_pass();

        frame.present();
    }
}
