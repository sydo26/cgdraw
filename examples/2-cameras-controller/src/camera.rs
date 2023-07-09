use std::f32::consts::PI;

use cgdraw::{
    angle::Deg,
    math::{perspective, Matrix4x4, Point3, Vec3},
    Camera, CameraAttributes,
};

#[derive(Clone, Copy, PartialEq)]
pub struct ExampleCamera {
    attributes: CameraAttributes<f32>,
    screen_width: u32,
    screen_height: u32,

    pub rotate_speed: f32,
    pub move_speed: f32,

    pub rotate: Vec3<f32>,
}

impl Default for ExampleCamera {
    fn default() -> Self {
        let pos = Point3 {
            x: 0.0,
            y: 0.0,
            z: 20.0,
        };

        let up = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let target = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        Self {
            attributes: CameraAttributes::new(up, pos, target),
            screen_width: 0,
            screen_height: 0,
            move_speed: (2.0 * PI) / 15.0,
            rotate_speed: (2.0 * PI) / 15.0,
            rotate: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl ExampleCamera {
    pub fn screen_resize(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}

impl Camera<f32> for ExampleCamera {
    fn calc_view_proj(&self) -> Matrix4x4<f32> {
        let look_at = Matrix4x4::look_at_rh(
            self.attributes.position,
            self.attributes.position - self.attributes.target,
            self.attributes.up,
        );

        let proj: Matrix4x4<f32> = perspective(
            self.screen_width as f32 / self.screen_height as f32,
            Deg(30.0).into(),
            0.1,
            100.0,
        );

        proj * look_at
    }
}
