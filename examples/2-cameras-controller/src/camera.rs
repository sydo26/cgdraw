use cgdraw::{
    angle::Deg,
    math::{Matrix4x4, Point3, Vec3},
    Camera, CameraAttributes, Projection,
};

pub struct ExampleCamera {
    pub projection: Projection<f32>,
    attributes: CameraAttributes<f32>,
}

impl Default for ExampleCamera {
    fn default() -> Self {
        let pos = Point3 {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let up = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let target = Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };

        Self {
            attributes: CameraAttributes::new(up, pos, target),
            projection: Projection::new(Deg(90.0), 0.1, 100.0),
        }
    }
}

impl Camera<f32> for ExampleCamera {
    fn calc_view_proj(&self) -> Matrix4x4<f32> {
        let look_at = Matrix4x4::look_at_rh(
            self.attributes.position,
            self.attributes.position - self.attributes.target,
            self.attributes.up,
        );

        self.projection.perspective() * look_at
    }
}
