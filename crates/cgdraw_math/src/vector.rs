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


    }
}

impl_vector!(Vec1 { x }, 1);
impl_vector!(Vec2 { x, y }, 2);
impl_vector!(Vec3 { x, y, z }, 3);
impl_vector!(Vec4 { x, y, z, w }, 4);
