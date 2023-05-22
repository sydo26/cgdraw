use cgdraw_core::keyboard::keys::VirtualKeyCode;
use winit::{dpi::PhysicalSize, event, event_loop};

pub enum WindowEvent {
    Resize { size: PhysicalSize<u32> },
    Close,
    Redraw,
    KeyPressed { key_code: VirtualKeyCode },
    KeyUp { key_code: VirtualKeyCode },
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
        fn input_window_events(event: &event::WindowEvent) -> Option<WindowEvent> {
            match event {
                event::WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            state: event::ElementState::Pressed,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => {
                    if let Some(vkc) = virtual_keycode {
                        VirtualKeyCode::by_winit_keycode(*vkc)
                            .map(|key_code| WindowEvent::KeyPressed { key_code })
                    } else {
                        None
                    }
                }

                event::WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            state: event::ElementState::Released,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => {
                    if let Some(vkc) = virtual_keycode {
                        VirtualKeyCode::by_winit_keycode(*vkc)
                            .map(|key_code| WindowEvent::KeyUp { key_code })
                    } else {
                        None
                    }
                }

                event::WindowEvent::Resized(physical_size) => Some(WindowEvent::Resize {
                    size: *physical_size,
                }),

                event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    Some(WindowEvent::Resize {
                        size: **new_inner_size,
                    })
                }

                _ => None,
            }
        }

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = event_loop::ControlFlow::Wait;

            match event {
                event::Event::MainEventsCleared => {
                    // Request redraw
                    self.window.request_redraw();
                }

                // Resize Event
                event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    let result = input_window_events(event);

                    if let Some(e) = result {
                        handler(e)
                    }

                    match event {
                        event::WindowEvent::CloseRequested
                        | event::WindowEvent::KeyboardInput {
                            input:
                                event::KeyboardInput {
                                    state: event::ElementState::Pressed,
                                    virtual_keycode: Some(event::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = event_loop::ControlFlow::Exit,

                        _ => {}
                    }
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
                    handler(WindowEvent::Redraw);
                }

                _ => (),
            }
        });
    }
}
