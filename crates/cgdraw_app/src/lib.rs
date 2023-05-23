use cgdraw_render::{Render, RenderState};
use cgdraw_state::State;
use cgdraw_ui::window::{Window, WindowEvent};
use events::AppEvent;
use graphics::Graphics;

pub mod events;
pub mod graphics;

pub struct App {
    window: Window,
    state: Option<State>,
}

impl Default for App {
    fn default() -> Self {
        let window = Window::default();

        Self {
            window,
            state: None,
        }
    }
}

impl App {
    async fn run_async<F>(mut self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        self.state = Some(State::new(&self.window.window).await);

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
