use crate::{
    angle::Rad,
    num::BaseFloat,
    vector::{Vec3, Vec4},
};

pub struct Matrix4x4<T> {
    pub c0: Vec4<T>,
    pub c1: Vec4<T>,
    pub c2: Vec4<T>,
    pub c3: Vec4<T>,
}

impl<T> Matrix4x4<T> {
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

    pub fn from_cols(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Self {
        Self { c0, c1, c2, c3 }
    }
}

impl<T: BaseFloat> Matrix4x4<T> {
    pub fn identity() -> Self {
        Self::from_cols(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    pub fn from_translate(xyz: Vec3<T>) -> Self {
        Self::from_cols(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(xyz.x, xyz.y, xyz.z, T::one()),
        )
    }

    pub fn from_rotate_x(angle: Rad<T>) -> Self {
        let (sin, cos) = angle.0.sin_cos();
        Self::from_cols(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), cos, sin, T::zero()),
            Vec4::new(T::zero(), -sin, cos, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    pub fn from_rotate_y(angle: Rad<T>) -> Self {
        let (sin, cos) = angle.0.sin_cos();
        Self::from_cols(
            Vec4::new(cos, T::zero(), -sin, T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(sin, T::zero(), cos, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    pub fn from_rotate_z(angle: Rad<T>) -> Self {
        let (sin, cos) = angle.0.sin_cos();
        Self::from_cols(
            Vec4::new(cos, sin, T::zero(), T::zero()),
            Vec4::new(-sin, cos, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    pub fn from_scale(x: T, y: T, z: T) -> Self {
        Self::from_cols(
            Vec4::new(x, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), y, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), z, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }
}
