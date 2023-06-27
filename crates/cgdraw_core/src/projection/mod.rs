use cgdraw_math::{angle::Rad, matrix::Matrix4x4, num::BaseFloat};
use num_traits::cast;

fn opengl_to_wgpu_matrix<T: BaseFloat>() -> Matrix4x4<T> {
    let half: T = cast(0.5).unwrap();

    #[rustfmt::skip]
    let matrix = Matrix4x4::new(
        T::one(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero(), T::zero(),
        T::zero(), T::zero(), half, T::zero(),
        T::zero(), T::zero(), half, T::one(),
    );

    matrix
}

pub fn perspective<T: BaseFloat>(aspect: T, fovy: Rad<T>, znear: T, zfar: T) -> Matrix4x4<T> {
    let two: T = cast(2).unwrap();
    let f = Rad::cot(fovy / two);

    let c0r0 = f / aspect;
    let c0r1 = T::zero();
    let c0r2 = T::zero();
    let c0r3 = T::zero();

    let c1r0 = T::zero();
    let c1r1 = f;
    let c1r2 = T::zero();
    let c1r3 = T::zero();

    let c2r0 = T::zero();
    let c2r1 = T::zero();
    let c2r2 = (zfar + znear) / (znear - zfar);
    let c2r3 = -T::one();

    let c3r0 = T::zero();
    let c3r1 = T::zero();
    let c3r2 = (two * zfar * znear) / (znear - zfar);
    let c3r3 = T::zero();

    #[rustfmt::skip]
    let perspective_matrix = Matrix4x4::new(
        c0r0, c0r1, c0r2, c0r3,
        c1r0, c1r1, c1r2, c1r3,
        c2r0, c2r1, c2r2, c2r3,
        c3r0, c3r1, c3r2, c3r3,
    );

    opengl_to_wgpu_matrix() * perspective_matrix
}

pub fn orthographic<T: BaseFloat>(
    left: T,
    right: T,
    bottom: T,
    top: T,
    znear: T,
    zfar: T,
) -> Matrix4x4<T> {
    let two: T = cast(2).unwrap();

    let c0r0 = two / (right - left);
    let c0r1 = T::zero();
    let c0r2 = T::zero();
    let c0r3 = T::zero();

    let c1r0 = T::zero();
    let c1r1 = two / (top - bottom);
    let c1r2 = T::zero();
    let c1r3 = T::zero();

    let c2r0 = T::zero();
    let c2r1 = T::zero();
    let c2r2 = -two / (zfar - znear);
    let c2r3 = T::zero();

    let c3r0 = -(right + left) / (right - left);
    let c3r1 = -(top + bottom) / (top - bottom);
    let c3r2 = -(zfar + znear) / (zfar - znear);
    let c3r3 = T::one();

    let ortho_matrix = Matrix4x4::new(
        c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2,
        c3r3,
    );

    opengl_to_wgpu_matrix() * ortho_matrix
}
