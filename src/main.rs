use cgdraw_app::{events::AppEvent, App};
use cgdraw_core::color::Color;

fn main() {
    let app = App::default();

    app.run(move |event| match event {
        AppEvent::Setup => {
            println!("Setup!")
        }

        AppEvent::Update => {}

        AppEvent::Draw { graphics } => {
            graphics.color(Color::CYAN);

            graphics.save_vertice(-0.5, 0.5, 0.0);
            graphics.save_vertice(0.5, 0.5, 0.0);
            graphics.save_vertice(0.5, -0.5, 0.0);

            graphics.draw([0, 1, 2, 2, 1, 0].to_vec());
        }

        AppEvent::KeyPressed { .. } => {
            println!("KeyPressed");
        }

        AppEvent::KeyUp { .. } => {
            println!("KeyUp");
        }

        _ => {}
    });
}
