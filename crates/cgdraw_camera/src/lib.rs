use cgdraw_core::projection::Projection;
use uniforms::CameraUniform;

pub mod uniforms;

pub struct Camera {
    /**
     * Define a posição da câmera no espaço
     */
    pub position: cgmath::Point3<f32>,

    /**
     * Define a rotação da câmera em torno do eixo Y
     */
    pub yaw: cgmath::Rad<f32>,

    /**
     * Define a rotação da câmera em torno do eixo X
     */
    pub pitch: cgmath::Rad<f32>,

    /**
     * Define o uniform da câmera
     */
    pub uniform: CameraUniform,
}

impl Camera {
    pub fn new<
        V: Into<cgmath::Point3<f32>>,
        Y: Into<cgmath::Rad<f32>>,
        P: Into<cgmath::Rad<f32>>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            uniform: CameraUniform::default(),
        }
    }
}

impl Camera {
    fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        use cgmath::InnerSpace;

        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        cgmath::Matrix4::look_to_rh(
            self.position,
            cgmath::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            cgmath::Vector3::unit_y(),
        )
    }
}

impl Camera {
    pub fn update(&mut self, projection: &Projection) {
        self.uniform.view_position = self.position.to_homogeneous().into();
        self.uniform.view_proj = (projection.to_wgpu_matrix() * self.calc_matrix()).into();
    }
}
