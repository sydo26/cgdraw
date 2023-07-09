use cameras_controller_example::ExampleCamera;
use cgdraw::{
    angle::Rad,
    event::AppEvent,
    graphics::{Graphics, Primitive},
    math::{Matrix4x4, Vec3},
    AppBuilder, Camera, Color,
};

/// Desenha um cubo 3D.
fn cube(g: &mut Graphics) {
    // Front Face
    g.begin(Primitive::TriangleList);
    g.color(Color::RED);
    g.v3d(-1.0, 1.0, 1.0);
    g.v3d(-1.0, -1.0, 1.0);
    g.v3d(1.0, -1.0, 1.0);
    g.v3d(1.0, 1.0, 1.0);
    g.indices([0, 1, 2, 0, 2, 3].to_vec());
    g.end();

    // Right Face
    g.begin(Primitive::TriangleList);
    g.color(Color::GRAY);
    g.v3d(1.0, 1.0, -1.0);
    g.v3d(1.0, -1.0, -1.0);
    g.v3d(1.0, -1.0, 1.0);
    g.v3d(1.0, 1.0, 1.0);
    g.indices([2, 1, 0, 3, 2, 0].to_vec());
    g.end();

    // Left Face
    g.begin(Primitive::TriangleList);
    g.color(Color::GOLD);
    g.v3d(-1.0, 1.0, -1.0);
    g.v3d(-1.0, -1.0, -1.0);
    g.v3d(-1.0, -1.0, 1.0);
    g.v3d(-1.0, 1.0, 1.0);
    g.indices([0, 1, 2, 0, 2, 3].to_vec());
    g.end();

    // Bottom Face
    g.begin(Primitive::TriangleList);
    g.color(Color::CYAN);
    g.v3d(-1.0, -1.0, -1.0);
    g.v3d(-1.0, -1.0, 1.0);
    g.v3d(1.0, -1.0, 1.0);
    g.v3d(1.0, -1.0, -1.0);
    g.indices([2, 1, 0, 3, 2, 0].to_vec());
    g.end();

    // Top Face
    g.begin(Primitive::TriangleList);
    g.color(Color::ORANGE);
    g.v3d(-1.0, 1.0, -1.0);
    g.v3d(-1.0, 1.0, 1.0);
    g.v3d(1.0, 1.0, 1.0);
    g.v3d(1.0, 1.0, -1.0);
    g.indices([0, 1, 2, 0, 2, 3].to_vec());
    g.end();

    // Back Face
    g.begin(Primitive::TriangleList);
    g.color(Color::VIOLET);
    g.v3d(-1.0, 1.0, -1.0);
    g.v3d(-1.0, -1.0, -1.0);
    g.v3d(1.0, -1.0, -1.0);
    g.v3d(1.0, 1.0, -1.0);
    g.indices([2, 1, 0, 2, 0, 3].to_vec());
    g.end();
}

/// Desenha linhas para melhorar a percepção do espaço 3D.
fn wireframe(g: &mut Graphics) {
    g.color(Color::GRAY);

    g.begin(Primitive::LineList);
    {
        for j in -10..=10 {
            g.v3d(-10.0, 0.0, j as f32);
            g.v3d(10.0, 0.0, j as f32);

            g.v3d(j as f32, 0.0, -10.0);
            g.v3d(j as f32, 0.0, 10.0);
        }
    }
    g.end();
}

/// Desenha o eixo de coordenadas.
fn draw_origin(g: &mut Graphics) {
    g.color(Color::BLUE);

    g.begin(Primitive::LineList);
    {
        g.v3d(0.0, 0.0, 0.0);
        g.v3d(0.0, 0.0, 1.0);
        g.indices([0, 1].to_vec());
    }
    g.end();

    g.color(Color::GREEN);

    g.begin(Primitive::LineList);
    {
        g.v3d(0.0, 0.0, 0.0);
        g.v3d(0.0, 1.0, 0.0);
        g.indices([0, 1].to_vec());
    }
    g.end();

    g.color(Color::RED);
    g.begin(Primitive::LineList);
    {
        g.v3d(0.0, 0.0, 0.0);
        g.v3d(1.0, 0.0, 0.0);
        g.indices([0, 1].to_vec());
    }
    g.end();
}

/// Função para desenhar os objetos na tela.
fn draw(g: &mut Graphics) {
    g.t.translate(0.0, 2.0, 0.0);
    cube(g);
    g.t.load_identity();

    g.t.scale(3.0, 3.0, 3.0);
    draw_origin(g);
    g.t.load_identity();

    wireframe(g);
}

fn main() {
    // Pega as configurações setadas pelo usuário e cria uma instância de APP
    let builder = AppBuilder::default();

    let app = builder.build();

    let mut camera_a = ExampleCamera::default();

    app.run(move |event| match event {
        AppEvent::Update {
            handler,
            delta_time,
        } => {
            let mut camera_view_proj = camera_a.calc_view_proj();

            let rotate_value = camera_a.rotate_speed * delta_time.as_secs_f32();

            camera_a.rotate = camera_a.rotate + Vec3::new(rotate_value, rotate_value, 0.0);

            camera_view_proj = camera_view_proj
                * Matrix4x4::from_rotate_y(Rad(camera_a.rotate.y))
                * Matrix4x4::from_rotate_x(Rad(camera_a.rotate.x));

            handler.camera_view_proj(camera_view_proj.into());
        }

        AppEvent::Resize { width, height } => {
            camera_a.screen_resize(width, height);
        }

        AppEvent::Draw { graphics } => {
            draw(graphics);
        }

        _ => {}
    });
}
