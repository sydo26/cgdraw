use cgdraw::{event::AppEvent, graphics::Primitive, AppBuilder, Color};

fn main() {
    let builder = AppBuilder::default();

    let app = builder.build();

    app.run(move |event| match event {
        AppEvent::Setup => {
            println!("Setup!")
        }

        AppEvent::Update { .. } => {}

        AppEvent::Draw { graphics } => {
            graphics.color(Color::GOLD);
            graphics.begin(Primitive::TriangleList);
            {
                graphics.v3d(-0.5, -0.5, 0.5);
                graphics.v3d(0.5, -0.5, 0.5);
                graphics.v3d(0.5, 0.5, 0.5);
                graphics.v3d(-0.5, 0.5, 0.5);

                graphics.indices([0, 1, 2, 2, 3, 0].to_vec());
            }
            graphics.end();
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
