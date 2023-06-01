use cgdraw_core::projection::Projection;
use cgmath::{InnerSpace};
use uniforms::CameraUniform;

pub mod uniforms;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub unique_id: &'static str,

    /// Define a posição da câmera
    pub position: cgmath::Point3<f32>,

    /// Define o ponto para onde a câmera deve apontar
    pub target: cgmath::Point3<f32>,

    /// Define o objeto que será enviado para o shader.
    pub uniform: CameraUniform,
}

impl Camera {
    pub fn new<
        V: Into<cgmath::Point3<f32>>,
        T: Into<cgmath::Point3<f32>>,
    >(
        unique_id: &'static str,
        position: V,
        target: T
    ) -> Self {
        Self {
            unique_id,
            position: position.into(),
            target: target.into(),
            uniform: CameraUniform::default(),
        }
    }
}

impl Camera {

    /// Rotaciona a câmera.
    pub fn rotate<V: Into<cgmath::Rad<f32>>, H: Into<cgmath::Rad<f32>>>(&mut self, yaw: V, pitch: H) {
        let (yaw_sin, yaw_cos) = yaw.into().0.sin_cos();
        let (pitch_sin, pitch_cos) = pitch.into().0.sin_cos();

        let direction_vector =
            cgmath::Vector3::new(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize();

        self.target = self.position - direction_vector;
    }

    /// Faz a câmera rotacionar no eixo vertical em graus.
    pub fn vertical_rotate<D: Into<cgmath::Rad<f32>>>(&mut self, deg: D) {
        self.rotate(deg, cgmath::Deg(0.0));
    }

    /// Faz a câmera rotacionar no eixo horizontal em graus.
    pub fn horizontal_rotate<D: Into<cgmath::Rad<f32>>>(&mut self, deg: D) {
        self.rotate(cgmath::Deg(0.0),deg );
    }
}

impl Camera {

    /// Calcula a matriz4x4 gerada pela câmera em right-handed
    fn look_to_rh(
        self,
        eye: cgmath::Point3<f32>,
        center: cgmath::Point3<f32>,
        up: cgmath::Vector3<f32>,
    ) -> cgmath::Matrix4<f32> {
        let forward = (center - eye).normalize();

        let side = forward.cross(up).normalize();

        let u = side.cross(forward);


        cgmath::Matrix4::new(
            // Line 01
            side.x, u.x, -forward.x, 0.0, //
            // Line 02
            side.y, u.y, -forward.y, 0.0, //
            // Line 03
            side.z, u.z, -forward.z, 0.0, //
            // Line 04
            0.0, 0.0, 0.0, 1.0,
        ) 
        
        // Translação
        * cgmath::Matrix4::new(
            // Line 01
            1.0, 0.0, 0.0, 0.0, //
            // Line 02
            0.0, 1.0, 0.0, 0.0, //
            // Line 03
            0.0, 0.0, 1.0, 0.0, //
            // Line 04
            -eye.x, -eye.y, -eye.z, 1.0,
        )
    }

    /// Calcula a matriz4x4 gerada pela câmera
    fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        self.look_to_rh(
            self.position,
            self.target,
            cgmath::Vector3::unit_y(),
        )
    }
}

impl Camera {
    
    /// Atualiza a câmera com base na projeção e a matriz de rotação e translação da câmera.
    pub fn update(&mut self, projection: &Projection) {
        self.uniform.view_position = self.position.to_homogeneous().into();
        self.uniform.view_proj = (projection.to_wgpu_matrix() * self.calc_matrix()).into();
    }
}
