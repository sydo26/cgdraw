use camera::CameraInitialAttributes;
use cgdraw_camera::Camera;
use cgdraw_core::projection::Projection;
use cgdraw_render::{Render, RenderState};
use cgdraw_state::State;
use cgdraw_ui::window::{Window, WindowEvent};
use events::AppEvent;
use graphics::Graphics;

pub mod camera;
pub mod events;
pub mod graphics;

pub struct App {
    window: Window,
    state: Option<State>,
    cameras: Vec<Camera>,
    current_camera: Option<Camera>,
    camera_projection: Option<Projection>,
}

impl Default for App {
    fn default() -> Self {
        let window = Window::default();

        Self {
            window,
            state: None,
            cameras: Vec::new(),
            current_camera: None,
            camera_projection: None,
        }
    }
}

impl App {
    async fn run_async<F>(mut self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        if self.cameras.is_empty() {
            panic!("Nenhuma câmera foi adicionada ao aplicativo!");
        }

        if self.current_camera.is_none() {
            panic!("Nenhuma câmera inicial foi definida!");
        }

        self.state = Some(State::new(&self.window.window, self.current_camera.unwrap()).await);

        self.window.run(move |window_event| match window_event {
            WindowEvent::Resumed => {
                event_handler(AppEvent::Setup);

                if let Some(state) = &mut self.state {
                    state
                        .surface
                        .configure(&state.device, &state.surface_config);
                }
            }

            WindowEvent::Redraw => {
                if let Some(state) = &mut self.state {
                    if let Some(proj) = &mut self.camera_projection {
                        proj.resize(state.surface_config.width, state.surface_config.height);
                        state.camera.update(proj);
                    }
                }

                event_handler(AppEvent::Update);
                if let Some(state) = &self.state {
                    let mut render = Render::new(state, RenderState::default());

                    let graphics = &mut Graphics::new(&mut render.render_state, state);
                    event_handler(AppEvent::Draw { graphics });

                    render.build();
                }
            }

            WindowEvent::Close => event_handler(AppEvent::Finished),

            WindowEvent::Resize { size } => event_handler(AppEvent::Resize {
                width: size.width,
                height: size.height,
            }),

            WindowEvent::KeyPressed { key_code } => {
                event_handler(AppEvent::KeyPressed { key_code })
            }

            WindowEvent::KeyUp { key_code } => event_handler(AppEvent::KeyUp { key_code }),
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

impl App {
    pub fn add_camera(
        mut self,
        unique_id: &'static str,
        attributes: CameraInitialAttributes,
    ) -> Self {
        self.cameras.push(Camera::new(
            unique_id,
            attributes.position,
            cgmath::Deg(attributes.v_rotation),
            cgmath::Deg(attributes.h_rotation),
        ));

        self
    }

    pub fn initial_camera(mut self, unique_id: &'static str) -> Self {
        if self.current_camera.is_some() {
            panic!("Você já definiu a câmera inicial!");
        }

        let camera = *self
            .cameras
            .iter()
            .find(|camera| camera.unique_id == unique_id)
            .unwrap();

        self.current_camera = Some(camera);

        self
    }

    pub fn camera_projection(mut self, projection: Projection) -> Self {
        self.camera_projection = Some(projection);

        self
    }
}
