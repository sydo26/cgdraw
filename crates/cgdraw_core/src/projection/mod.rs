use cgdraw_math::{angle::Rad, matrix::Matrix4x4, num::BaseFloat};
use num_traits::cast;

pub struct Projection<T> {
    /**
     * Aspecto que define a proporção
     * entre a largura e a altura da tela
     */
    pub aspect: T,

    /**
     * Define o ângulo de visão da projeção
     */
    pub fovy: Rad<T>,

    /**
     * Define a distância do plano de visão
     * mais próximo da origem da projeção
     */
    pub znear: T,

    /**
     * Define a distância do plano de visão
     * mais distante da origem da projeção
     */
    pub zfar: T,
}

impl<T: BaseFloat> Projection<T> {
    pub fn new<F: Into<Rad<T>>>(fovy: F, znear: T, zfar: T) -> Self {
        Self {
            aspect: T::one(),
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = T::from(width).unwrap() / T::from(height).unwrap();
    }

    pub fn perspective(&self) -> Matrix4x4<T> {
        let half: T = cast(0.5).unwrap();
        #[rustfmt::skip]
        let opengl_to_wgpu_matrix = Matrix4x4::new(
            T::one(), T::zero(), T::zero(), T::zero(),
            T::zero(), T::one(), T::zero(), T::zero(),
            T::zero(), T::zero(), half, T::zero(),
            T::zero(), T::zero(), half, T::one(),
        );

        let two: T = cast(2).unwrap();
        let frad: Rad<T> = self.fovy / two;
        let f = Rad::cot(frad);

        let c0r0 = f / self.aspect;
        let c0r1 = T::zero();
        let c0r2 = T::zero();
        let c0r3 = T::zero();

        let c1r0 = T::zero();
        let c1r1 = f;
        let c1r2 = T::zero();
        let c1r3 = T::zero();

        let c2r0 = T::zero();
        let c2r1 = T::zero();
        let c2r2 = (self.zfar + self.znear) / (self.znear - self.zfar);
        let c2r3 = -T::one();

        let c3r0 = T::zero();
        let c3r1 = T::zero();
        let c3r2 = (two * self.zfar * self.znear) / (self.znear - self.zfar);
        let c3r3 = T::zero();

        let perspective_matrix = Matrix4x4::new(
            c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1,
            c3r2, c3r3,
        );

        opengl_to_wgpu_matrix * perspective_matrix
    }
}
