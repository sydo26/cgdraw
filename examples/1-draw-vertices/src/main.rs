use cgdraw::{event::AppEvent, AppBuilder, Color};

fn main() {
    let builder = AppBuilder::default();

    let app = builder.build();

    app.run(move |event| match event {
        AppEvent::Setup => {
            println!("Setup!")
        }

        AppEvent::Update { .. } => {}

        AppEvent::Draw { graphics } => {
            graphics.color(Color::RED);
            graphics.save_vertice(-0.5, 0.5, 0.0);
            graphics.color(Color::YELLOW);
            graphics.save_vertice(0.5, 0.5, 0.0);
            graphics.color(Color::BLUE);
            graphics.save_vertice(0.5, -0.5, 0.0);
            graphics.color(Color::SILVER);
            graphics.save_vertice(-0.5, -0.5, 0.0);

            graphics.draw(
                [
                    0, 1, 2, 2, 1, 0, // TRIANGULO 01
                    0, 2, 3, 3, 2, 0, // TRIANGULO 02
                ]
                .to_vec(),
            );
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
