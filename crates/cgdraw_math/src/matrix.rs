use crate::{
    angle::Rad,
    num::BaseFloat,
    point::Point3,
    vector::{Vec3, Vec4},
};

use std::mem;

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
    /// Cria uma matriz de transformação que mira a partir de um ponto de origem `eye`, para um
    /// vetor de direção `dir`, usando o vetor `up` como orientação de referência.
    pub fn look_at_rh(eye: Point3<T>, direction: Vec3<T>, up: Vec3<T>) -> Matrix4x4<T> {
        // O vetor direção representa o eixo Z da câmera. Para sabermos qual eixo
        let dir = direction.normalize();

        // O vetor direito representa o eixo horizontal da câmera. Para sabermos
        // qual eixo é o horizontal, precisamos de um vetor indicando qual é o eixo
        // vertical da câmera. Nesse caso, o vetor up. Se o vetor up for o eixo Y,
        // o meu eixo horizontal será o eixo X.
        let right = up.cross(dir).normalize();

        // Aqui basicamente estamos capturando o vetor up da câmera. O vetor up
        // é o vetor que aponta para cima, ou seja, o eixo Y.
        let up = dir.cross(right);

        // Look-at right handed matrix
        //  rx   ry   rz  0
        //  ux   uy   uz  0
        //  dx   dy   dz  0
        //  0    0    0   1
        let look_at_matrix = Matrix4x4::new(
            // Line 01
            right.x,
            up.x,
            dir.x,
            T::zero(), //
            // Line 02
            right.y,
            up.y,
            dir.y,
            T::zero(),
            // Line 03
            right.z,
            up.z,
            dir.z,
            T::zero(), //
            // Line 04
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        );

        // Multiplicação da matriz de look-at com a matriz de translação
        look_at_matrix * Self::from_translate(Vec3::new(-eye.x, -eye.y, -eye.z))
    }

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
    ///     [0, 0,   0,    1],
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
            Vec4::new(cos, T::zero(), sin, T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(-sin, T::zero(), cos, T::zero()),
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

// macro_rules! impl_matrix {
//     ($MatrixN:ident, $VectorN:ident { $($field:ident : $row_index:expr),+ }) => {};
// }

macro_rules! fixed_array_conversions {
    ($MatrixN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        #[allow(clippy::from_over_into)]
        impl<$S> Into<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn into(self) -> [[$S; $n]; $n] {
                match self { $MatrixN { $($field),+ } => [$($field.into()),+] }
            }
        }

        impl<$S> AsRef<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn as_ref(&self) -> &[[$S; $n]; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [[$S; $n]; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Copy> From<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn from(m: [[$S; $n]; $n]) -> $MatrixN<$S> {
                // We need to use a copy here because we can't pattern match on arrays yet
                $MatrixN { $($field: From::from(m[$index])),+ }
            }
        }

        impl<'a, $S> From<&'a [[$S; $n]; $n]> for &'a $MatrixN<$S> {
            #[inline]
            fn from(m: &'a [[$S; $n]; $n]) -> &'a $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        impl<'a, $S> From<&'a mut [[$S; $n]; $n]> for &'a mut $MatrixN<$S> {
            #[inline]
            fn from(m: &'a mut [[$S; $n]; $n]) -> &'a mut $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        impl<$S> AsRef<[$S; ($n * $n)]> for $MatrixN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; ($n * $n)] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; ($n * $n)]> for $MatrixN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; ($n * $n)] {
                unsafe { mem::transmute(self) }
            }
        }


        impl<'a, $S> From<&'a [$S; ($n * $n)]> for &'a $MatrixN<$S> {
            #[inline]
            fn from(m: &'a [$S; ($n * $n)]) -> &'a $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; ($n * $n)]> for &'a mut $MatrixN<$S> {
            #[inline]
            fn from(m: &'a mut [$S; ($n * $n)]) -> &'a mut $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }
    }
}

fixed_array_conversions!(Matrix4x4<T> { c0: 0, c1: 1, c2: 2, c3: 3 }, 4);
