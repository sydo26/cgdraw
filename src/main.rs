use cgdraw_app::{events::AppEvent, App};

fn main() {
    let app = App::default();

    app.run(move |event| match event {
        AppEvent::Setup => {
            println!("Setup!")
        }

        AppEvent::Update => {}

        AppEvent::Draw => {}

        AppEvent::KeyPressed { .. } => {
            println!("KeyPressed");
        }

        AppEvent::KeyUp { .. } => {
            println!("KeyUp");
        }

        _ => {
            println!("Default");
        }
    });
}
