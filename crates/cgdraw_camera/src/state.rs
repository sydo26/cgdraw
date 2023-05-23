use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};

pub struct CameraState {
    /**
     * Define a posição da câmera no espaço
     */
    pub position: Point3<f32>,

    /**
     * Define a rotação da câmera em torno do eixo Y
     */
    pub yaw: Rad<f32>,

    /**
     * Define a rotação da câmera em torno do eixo X
     */
    pub pitch: Rad<f32>,
}

impl CameraState {
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }
}
