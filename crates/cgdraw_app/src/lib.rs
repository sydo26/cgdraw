use cgdraw_state::State;
use cgdraw_ui::window::{Window, WindowEvent};
use events::AppEvent;

pub mod events;

pub struct App {
    window: Window,
}

impl Default for App {
    fn default() -> Self {
        let window = Window::default();

        Self { window }
    }
}

impl App {
    async fn run_async<F>(self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        // let state = State::new(&self.window.window).await;

        self.window.run(move |window_event| match window_event {
            WindowEvent::Redraw => {
                event_handler(AppEvent::Update);
                event_handler(AppEvent::Draw);
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
