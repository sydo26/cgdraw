use std::iter::once;

mod pipeline;

pub use pipeline::*;

pub struct Render<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> Render<'a> {
    pub fn new(device: &'a wgpu::Device, queue: &'a wgpu::Queue) -> Self {
        Self { device, queue }
    }
}

impl<'a> Render<'a> {
    pub fn render<F: 'static + FnMut(wgpu::RenderPass)>(
        self,
        default_view: &'a wgpu::TextureView,
        depth_view: &'a wgpu::TextureView,
        mut render_handler: F,
    ) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        let desc = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: default_view,
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
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        };

        {
            let render_pass = encoder.begin_render_pass(&desc);

            render_handler(render_pass);
        }

        self.queue.submit(once(encoder.finish()));
    }
}
