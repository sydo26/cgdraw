use cgdraw_math::{matrix::Matrix4x4, num::BaseFloat, point::Point3, vector::Vec3};

pub mod uniform;

pub trait Camera<T: BaseFloat> {
    /// Calcula a matriz 4x4 de visualização da projeção da câmera no espaço global.
    fn calc_view_proj(&mut self) -> Matrix4x4<T>;
}

#[derive(Debug, Clone, Copy)]
pub struct CameraAttributes<T> {
    /// É o vetor que define a direção para cima da câmera.
    pub up: Vec3<T>,

    /// É o ponto onde marca a posição da câmera nos eixos (x,y,z)
    pub position: Point3<T>,

    /// É o ponto para onde a câmera deve apontar nos eixos (x,y,z)
    pub target: Point3<T>,
}

impl<T: BaseFloat> CameraAttributes<T> {
    /// Cria uma nova instância de `CameraAttributes`.
    pub fn new(up: Vec3<T>, position: Point3<T>, target: Point3<T>) -> Self {
        Self {
            up,
            position,
            target,
        }
    }
}

impl<T: BaseFloat> Default for CameraAttributes<T> {
    fn default() -> Self {
        Self {
            up: Vec3::new(T::zero(), T::one(), T::zero()),
            position: Point3::new(T::zero(), T::zero(), T::zero()),
            target: Point3::new(T::zero(), T::zero(), T::zero()),
        }
    }
}
