use std::fmt;
use std::mem;
use std::ops::*;

use num_traits::{Float, Zero};

use crate::num::BaseNum;

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vec1<T> {
    pub x: T,
}
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: BaseNum> Vec3<T> {
    /// Faz o produto cruzado entre dois vetores.
    #[inline]
    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }

    /// Cria um `Vec4` com os valores do vetor tridimensional e o valor `w`.
    #[inline]
    pub fn extend(self, w: T) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Cria um `Vec2` com os valores do vetor tridimensional, removendo o valor `z`.
    #[inline]
    pub fn truncate(self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }
}

impl<T: BaseNum> Vec4<T> {
    /// Cria um `Vec3` com os valores do vetor quadradimensional, removendo o valor `w`.
    #[inline]
    pub fn truncate(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

macro_rules! impl_vector {
    ($V:ident { $($field:ident),+ }, $n:expr) => {
        impl<T> $V<T> {
            #[inline]
            pub const fn new($($field: T),+) -> $V<T> {
                $V { $($field),+ }
            }
        }

        impl<T: Copy> $V<T> {
            #[inline]
            pub fn len() -> usize {
                $n
            }

            #[inline]
            pub fn is_finite(&self) -> bool where T: Float {
                $(self.$field.is_finite())&&+
            }

            #[inline]
            pub fn from_value(scalar: T) -> $V<T> {
                $V { $($field: scalar),+ }
            }

            #[inline]
            pub fn sum(self) -> T where T: Add<Output = T> {
                fold_array!(add, { $(self.$field),+ })
            }
        }

        impl<T: BaseNum + Neg<Output = T>> Neg for $V<T>  {
            type Output = $V<T>;

            #[inline]
            fn neg(self) -> $V<T> {
                $V::new($(-self.$field),+)
            }
        }

        impl<T: BaseNum> Add<$V<T> > for $V<T> {
            type Output = $V<T>;

            #[inline]
            fn add(self, other: $V<T>) -> $V<T> {
                $V::new($(self.$field + other.$field),+)
            }
        }

        impl<T: BaseNum> Sub<$V<T> > for $V<T> {
            type Output = $V<T>;

            #[inline]
            fn sub(self, other: $V<T>) -> $V<T> {
                $V::new($(self.$field - other.$field),+)
            }
        }

        impl<T: BaseNum> Div<$V<T> > for $V<T> {
            type Output = $V<T>;

            #[inline]
            fn div(self, other: $V<T>) -> $V<T> {
                $V::new($(self.$field / other.$field),+)
            }
        }

        impl<T: BaseNum> Mul<$V<T> > for $V<T> {
            type Output = $V<T>;

            #[inline]
            fn mul(self, other: $V<T>) -> $V<T> {
                $V::new($(self.$field * other.$field),+)
            }
        }

        impl<T: BaseNum> Zero for $V<T> {
            #[inline]
            fn zero() -> $V<T> {
                $V::from_value(T::zero())
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == $V::zero()
            }
        }

        impl<T: BaseNum> Mul<T> for $V<T> {
            type Output = $V<T>;

            fn mul(self, rhs: T) -> Self::Output {
                $V::new($(self.$field * rhs),+)
            }
        }

        impl<T: BaseNum> Div<T> for $V<T> {
            type Output = $V<T>;

            fn div(self, rhs: T) -> Self::Output {
                $V::new($(self.$field / rhs),+)
            }
        }

        impl<T: BaseNum> Add<T> for $V<T> {
            type Output = $V<T>;

            fn add(self, rhs: T) -> Self::Output {
                $V::new($(self.$field + rhs),+)
            }
        }

        impl<T: BaseNum> Sub<T> for $V<T> {
            type Output = $V<T>;

            fn sub(self, rhs: T) -> Self::Output {
                $V::new($(self.$field - rhs),+)
            }
        }
    }
}

impl_vector!(Vec1 { x }, 1);
impl_vector!(Vec2 { x, y }, 2);
impl_vector!(Vec3 { x, y, z }, 3);
impl_vector!(Vec4 { x, y, z, w }, 4);

impl_fixed_array_conversions!(Vec1<S> { x: 0 }, 1);
impl_fixed_array_conversions!(Vec2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Vec3<S> { x: 0, y: 1, z: 2 }, 3);
impl_fixed_array_conversions!(Vec4<S> { x: 0, y: 1, z: 2, w: 3 }, 4);

impl_tuple_conversions!(Vec1<S> { x }, (S,));
impl_tuple_conversions!(Vec2<S> { x, y }, (S, S,));
impl_tuple_conversions!(Vec3<S> { x, y, z }, (S, S, S,));
impl_tuple_conversions!(Vec4<S> { x, y, z, w }, (S, S, S, S,));

impl<S: fmt::Debug> fmt::Debug for Vec1<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec1 ")?;
        <[S; 1] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vec2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2 ")?;
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vec3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3 ")?;
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vec4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec4 ")?;
        <[S; 4] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}
