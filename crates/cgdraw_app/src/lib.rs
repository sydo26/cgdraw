use cgdraw_ui::window::{Window, WindowEvent};
use events::AppEvent;

pub mod events;

pub struct App {
    window: Window,
}

impl App {
    #[inline]
    pub fn run<F>(self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(AppEvent),
    {
        self.window.run(move |window_event| match window_event {
            WindowEvent::Redraw => {
                event_handler(AppEvent::Update);
            }

            WindowEvent::Close => event_handler(AppEvent::Finished),

            WindowEvent::RedrawEventsCleared => {}

            WindowEvent::Resize { size } => event_handler(AppEvent::Resize {
                width: size.width,
                height: size.height,
            }),
        })
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: Window::default(),
        }
    }
}
