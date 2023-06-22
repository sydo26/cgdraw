use std::time::Instant;

use cgdraw_camera::uniform::CameraUniformFloat32;
use cgdraw_core::graphic::Texture;
use cgdraw_render::{Render, RenderState};
use cgdraw_state::State;
use cgdraw_ui::window::{Window, WindowEvent};
use events::AppEvent;

use crate::{graphics::Graphics, handler::AppHandler};

pub mod builder;
pub mod events;
pub mod graphics;
pub mod handler;

pub struct App {}

impl App {
    async fn run_async<F>(self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        let window = Window::default();

        let mut state = State::new(&window.window, CameraUniformFloat32::default()).await;

        let mut last_render_time = Instant::now();

        window.run(move |window_event| match window_event {
            WindowEvent::Resumed => {
                event_handler(AppEvent::Setup);

                state
                    .surface
                    .configure(&state.device, &state.surface_config);
            }

            WindowEvent::Redraw => {
                let now = Instant::now();
                let delta_time = now - last_render_time;
                last_render_time = now;

                let handler = &mut AppHandler::new(&mut state);

                event_handler(AppEvent::Update {
                    handler,
                    delta_time,
                });

                let mut render = Render::new(&state, RenderState::default());

                let graphics = &mut Graphics::new(&mut render.render_state, &state);
                event_handler(AppEvent::Draw { graphics });

                render.build();
            }

            WindowEvent::Close => event_handler(AppEvent::Finished),

            WindowEvent::Resize { size } => {
                if size.width > 0 && size.height > 0 {
                    state.surface_config.width = size.width;
                    state.surface_config.height = size.height;
                    state
                        .surface
                        .configure(&state.device, &state.surface_config);
                    state.depth_view =
                        Texture::create_depth_texture(&state.device, &state.surface_config).view;

                    event_handler(AppEvent::Resize {
                        width: size.width,
                        height: size.height,
                    });
                }
            }

            WindowEvent::KeyPressed { key_code } => {
                event_handler(AppEvent::KeyPressed { key_code })
            }

            WindowEvent::KeyUp { key_code } => event_handler(AppEvent::KeyUp { key_code }),
        })
    }

    #[inline]
    pub fn run<F>(mut self, event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        pollster::block_on(self.run_async(event_handler));
    }
}
