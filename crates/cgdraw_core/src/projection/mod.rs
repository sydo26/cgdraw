use cgmath::Rad;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Projection {
    /**
     * Aspecto que define a proporção
     * entre a largura e a altura da tela
     */
    pub aspect: f32,

    /**
     * Define o ângulo de visão da projeção
     */
    pub fovy: Rad<f32>,

    /**
     * Define a distância do plano de visão
     * mais próximo da origem da projeção
     */
    pub znear: f32,

    /**
     * Define a distância do plano de visão
     * mais distante da origem da projeção
     */
    pub zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn to_wgpu_matrix(&self) -> cgmath::Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}
