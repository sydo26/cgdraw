use crate::{
    angle::Rad,
    num::BaseFloat,
    vector::{Vec3, Vec4},
};

/// Matriz 4x4, onde cada coluna é um vetor de 4 elementos.
pub struct Matrix4x4<T> {
    pub c0: Vec4<T>,
    pub c1: Vec4<T>,
    pub c2: Vec4<T>,
    pub c3: Vec4<T>,
}

impl<T> Matrix4x4<T> {
    /// Cria uma matriz 4x4 a partir de 16 elementos.
    #[inline]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        c0r0: T, c0r1: T, c0r2: T, c0r3: T,
        c1r0: T, c1r1: T, c1r2: T, c1r3: T,
        c2r0: T, c2r1: T, c2r2: T, c2r3: T,
        c3r0: T, c3r1: T, c3r2: T,c3r3: T,
    ) -> Self {
        Self::from_cols(
            Vec4::new(c0r0, c0r1, c0r2, c0r3),
            Vec4::new(c1r0, c1r1, c1r2, c1r3),
            Vec4::new(c2r0, c2r1, c2r2, c2r3),
            Vec4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    /// Cria uma matriz 4x4 a partir de 4 vetores de 4 elementos.
    pub fn from_cols(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Self {
        Self { c0, c1, c2, c3 }
    }
}

impl<T: BaseFloat> Matrix4x4<T> {
    /// Cria uma matriz identidade.
    pub fn identity() -> Self {
        Self::from_cols(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Cria uma matriz 4x4 de translação a partir de um vetor de 3 elementos (x, y, z).
    pub fn from_translate(xyz: Vec3<T>) -> Self {
        Self::from_cols(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(xyz.x, xyz.y, xyz.z, T::one()),
        )
    }

    /// Cria uma matriz 4x4 de rotação do eixo X a partir de um ângulo em radianos.
    /// ```rust
    /// // Matriz Transposta
    /// [
    ///     [1, 0,   0,    0],
    ///     [0, cos, sin,  0],
    ///     [0, -sin, cos, 0],
    ///     [0, 0,   0,    0],
    /// ]
    /// ```
    pub fn from_rotate_x(angle: Rad<T>) -> Self {
        let (sin, cos) = angle.0.sin_cos();
        Self::from_cols(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), cos, sin, T::zero()),
            Vec4::new(T::zero(), -sin, cos, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Cria uma matriz 4x4 de rotação do eixo Y a partir de um ângulo em radianos.
    /// ```rust
    /// // Matriz Transposta
    /// [
    ///     [cos,  0, sin,  0],
    ///     [0,    1, 0,    0],
    ///     [-sin, 0, cos,  0],
    ///     [0,    0, 0,    1],
    /// ]
    /// ```
    pub fn from_rotate_y(angle: Rad<T>) -> Self {
        let (sin, cos) = angle.0.sin_cos();
        Self::from_cols(
            Vec4::new(cos, T::zero(), -sin, T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(sin, T::zero(), cos, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Cria uma matriz 4x4 de rotação do eixo Z a partir de um ângulo em radianos.
    /// ```rust
    /// // Matriz Transposta
    /// [
    ///     [cos,  sin, 0, 0],
    ///     [-sin, cos, 0, 0],
    ///     [0,    0,   1, 0],
    ///     [0,    0,   0, 1],
    /// ]
    /// ```
    pub fn from_rotate_z(angle: Rad<T>) -> Self {
        let (sin, cos) = angle.0.sin_cos();
        Self::from_cols(
            Vec4::new(cos, sin, T::zero(), T::zero()),
            Vec4::new(-sin, cos, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Cria uma matriz 4x4 de escala a partir de um vetor de 3 elementos (x, y, z).
    /// ```rust
    /// // Matriz Transposta
    /// [
    ///     [x, 0, 0, 0],
    ///     [0, y, 0, 0],
    ///     [0, 0, z, 0],
    ///     [0, 0, 0, 1],
    /// ]
    /// ```
    pub fn from_scale(xyz: Vec3<T>) -> Self {
        Self::from_cols(
            Vec4::new(xyz.x, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), xyz.y, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), xyz.z, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }
}
