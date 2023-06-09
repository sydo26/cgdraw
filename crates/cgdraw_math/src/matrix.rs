use crate::{
    angle::Rad,
    num::BaseFloat,
    point::Point3,
    vector::{Vec3, Vec4},
};

use std::ops::*;

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
    /// Retorna a linha `r` da matriz.
    #[inline]
    fn row(&self, r: usize) -> Vec4<T> {
        let x = self[0][r];
        let y = self[1][r];
        let z = self[2][r];
        let w = self[3][r];

        Vec4::new(x, y, z, w)
    }

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

        let translation_matrix = Self::from_translate(Vec3::new(-eye.x, -eye.y, -eye.z));

        // Multiplicação da matriz de look-at com a matriz de translação
        look_at_matrix * translation_matrix
    }

    /// Cria uma matriz identidade.
    pub fn identity() -> Matrix4x4<T> {
        Self::from_cols(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Cria uma matriz 4x4 de translação a partir de um vetor de 3 elementos (x, y, z).
    pub fn from_translate(xyz: Vec3<T>) -> Matrix4x4<T> {
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
    pub fn from_rotate_x(angle: Rad<T>) -> Matrix4x4<T> {
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
    pub fn from_rotate_y(angle: Rad<T>) -> Matrix4x4<T> {
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
    pub fn from_rotate_z(angle: Rad<T>) -> Matrix4x4<T> {
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
    pub fn from_scale(xyz: Vec3<T>) -> Matrix4x4<T> {
        Self::from_cols(
            Vec4::new(xyz.x, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), xyz.y, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), xyz.z, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }
}

// Implementa a multiplicação de matrizes 4x4.
impl<T: BaseFloat> Mul<Matrix4x4<T>> for Matrix4x4<T> {
    type Output = Matrix4x4<T>;
    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];

        Matrix4x4::from_cols(
            a * rhs[0][0] + b * rhs[0][1] + c * rhs[0][2] + d * rhs[0][3],
            a * rhs[1][0] + b * rhs[1][1] + c * rhs[1][2] + d * rhs[1][3],
            a * rhs[2][0] + b * rhs[2][1] + c * rhs[2][2] + d * rhs[2][3],
            a * rhs[3][0] + b * rhs[3][1] + c * rhs[3][2] + d * rhs[3][3],
        )
    }
}

macro_rules! impl_matrix {
    ($MatrixN:ident { $($field:ident),+ }, $n:expr) => {
        impl<T: BaseFloat> Mul<T> for $MatrixN<T> {
            type Output = $MatrixN<T>;

            #[inline]
            fn mul(self, rhs: T) -> $MatrixN<T> {
                $MatrixN { $($field: self.$field * rhs),+ }
            }
        }
    };
}

macro_rules! impl_matrix_vector_mul {
    ($MatrixN:ident, $VectorN:ident { $($field:ident : $row_index:expr),+} ) => {
        impl<T: BaseFloat> Mul<$VectorN<T>> for $MatrixN<T> {
            type Output = $VectorN<T>;

            #[inline]
            fn mul(self, vector: $VectorN<T>) -> $VectorN<T> {
                $VectorN::new($(self.row($row_index).dot(vector.clone())),+)
            }
        }
    };
}

macro_rules! index_operators {
    ($MatrixN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $MatrixN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&self, i: $I) -> &$Output {
                let v: &[[$S; $n]; $n] = self.as_ref();
                From::from(&v[i])
            }
        }

        impl<$S> IndexMut<$I> for $MatrixN<$S> {
            #[inline]
            fn index_mut<'a>(&mut self, i: $I) -> &mut $Output {
                let v: &mut [[$S; $n]; $n] = self.as_mut();
                From::from(&mut v[i])
            }
        }
    };
}

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

impl_matrix!(Matrix4x4 { c0, c1, c2, c3 }, 4);

impl_matrix_vector_mul!(
    Matrix4x4,
    Vec4 {
        x: 0,
        y: 1,
        z: 2,
        w: 3
    }
);

fixed_array_conversions!(Matrix4x4<T> { c0: 0, c1: 1, c2: 2, c3: 3 }, 4);

index_operators!(Matrix4x4<T>, 4, Vec4<T>, usize);
