use cgdraw::{event::AppEvent, App};

fn main() {
    let app = App::default();

    app.run(move |event| match event {
        AppEvent::Setup => {
            println!("Setup!")
        }

        AppEvent::Update => {}

        AppEvent::Draw { .. } => {}

        AppEvent::KeyPressed { .. } => {}

        AppEvent::KeyUp { .. } => {}

        _ => {}
    });
}
