use cgdraw::{event::AppEvent, App, CameraInitialAttributes, Color, Projection};

fn main() {
    let app = App::default();

    app.add_camera(
        "camera",
        CameraInitialAttributes {
            position: (0.0, 0.0, -1.0),
            v_rotation: 90.0,
            h_rotation: 0.0,
        },
    )
    .initial_camera("camera")
    .camera_projection(Projection::new(120.0, 0.1, 100.0))
    .run(move |event| match event {
        AppEvent::Setup => {
            println!("Setup!")
        }

        AppEvent::Update => {}

        AppEvent::Draw { graphics } => {
            graphics.color(Color::RED);
            graphics.save_vertice(-0.5, 0.5, 0.0);
            graphics.save_vertice(0.5, 0.5, 0.0);
            graphics.color(Color::BLUE);
            graphics.save_vertice(0.5, -0.5, 0.0);
            graphics.save_vertice(-0.5, -0.5, 0.0);

            graphics.draw([0, 1, 2, 2, 1, 0, 0, 2, 3, 3, 2, 0].to_vec());
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
