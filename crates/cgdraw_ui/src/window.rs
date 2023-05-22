use winit::{dpi::PhysicalSize, event, event_loop};

pub enum WindowEvent {
    RedrawEventsCleared,
    Resize { size: PhysicalSize<u32> },
    Close,
    Redraw,
}

pub struct Window {
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub window: winit::window::Window,
}

impl Default for Window {
    fn default() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title("CG Draw")
            .build(&event_loop)
            .unwrap();

        Self { window, event_loop }
    }
}

impl Window {
    pub fn run<F>(self, mut handler: F) -> !
    where
        F: 'static + FnMut(WindowEvent),
    {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = event_loop::ControlFlow::Wait;

            match event {
                event::Event::RedrawEventsCleared => {
                    // Request redraw
                    self.window.request_redraw();
                }

                // Resize Event
                event::Event::WindowEvent {
                    event:
                        event::WindowEvent::Resized(size)
                        | event::WindowEvent::ScaleFactorChanged {
                            new_inner_size: &mut size,
                            ..
                        },
                    window_id,
                } if window_id == self.window.id() => {
                    // Resize
                    handler(WindowEvent::Resize { size });
                }

                // Close Event
                event::Event::WindowEvent {
                    event: event::WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => {
                    // builder.close_requested();
                    *control_flow = event_loop::ControlFlow::Exit;
                }

                // Redraw request
                event::Event::RedrawRequested(_) => {
                    // builder.redraw_requested();
                }

                _ => (),
            }
        });
    }
}
