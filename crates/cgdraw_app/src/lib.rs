use std::{collections::HashMap, time::Instant};

use cgdraw_camera::{uniform::CameraUniformFloat32, Camera};
use cgdraw_core::graphic::Texture;
use cgdraw_render::{Render, RenderState};
use cgdraw_state::State;
use cgdraw_ui::window::{Window, WindowEvent};
use events::AppEvent;
use graphics::Graphics;
use handler::AppHandler;
use uuid::Uuid;

pub mod events;
pub mod graphics;
pub mod handler;

struct AppCamera {
    camera: Box<dyn Camera<f32> + 'static>,
    selected: bool,
}

impl AppCamera {
    #[inline]
    fn update_camera(&mut self, state: &mut State) {
        let view_proj = self.camera.calc_view_proj();

        // state.camera_uniform.view_position = [0.0; 4];
        state.camera_uniform.view_proj = view_proj.into();
    }
}

pub struct AppBuilder {
    cameras: HashMap<String, AppCamera>,
}

impl AppBuilder {
    /// Adiciona uma nova implementação de câmera ao aplicativo.
    #[inline]
    pub fn add_camera(mut self, camera: impl Camera<f32> + 'static) {
        self.cameras.insert(
            Uuid::new_v4().to_string(),
            AppCamera {
                camera: Box::new(camera),
                selected: self.cameras.is_empty(),
            },
        );
    }

    /// Constrói o aplicativo.
    #[inline]
    pub fn build(self, window: Window) -> App {
        if self.cameras.is_empty() {
            panic!("Nenhuma câmera foi adicionada ao aplicativo!");
        }

        App {
            window,
            cameras: self.cameras,
        }
    }
}
pub struct App {
    window: Window,
    cameras: HashMap<String, AppCamera>,
}

impl App {
    async fn run_async<F>(mut self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        let mut state = State::new(&self.window.window, CameraUniformFloat32::default()).await;

        let mut last_render_time = Instant::now();

        self.window.run(move |window_event| {
            fn update_camera(cameras: &mut HashMap<String, AppCamera>, state: &mut State) {
                cameras
                    .iter_mut()
                    .find(|(_, camera)| camera.selected)
                    .as_mut()
                    .unwrap()
                    .1
                    .update_camera(state);
            }

            match window_event {
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

                    //     proj.resize(state.surface_config.width, state.surface_config.height);

                    // Update Camera
                    update_camera(&mut self.cameras, &mut state);

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
                            Texture::create_depth_texture(&state.device, &state.surface_config)
                                .view;

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
            }
        })
    }

    #[inline]
    pub fn run<F>(self, event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        pollster::block_on(self.run_async(event_handler));
    }
}
