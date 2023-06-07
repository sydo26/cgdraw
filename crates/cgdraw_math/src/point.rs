use crate::num::BaseNum;
use crate::vector::{Vec1, Vec2, Vec3, Vec4};
use std::fmt;
use std::mem;

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point1<T> {
    pub x: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: BaseNum> Point3<T> {
    #[inline]
    pub fn from_homogeneous(v: Vec4<T>) -> Point3<T> {
        let vec = v.truncate() * (T::one() / v.w);
        Point3::new(vec.x, vec.y, vec.z)
    }

    #[inline]
    pub fn to_homogeneous(self) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, T::one())
    }
}

macro_rules! impl_point {
    ($P:ident { $($field:ident),+ }, $VectorN:ident, $n:expr) => {
        impl<T> $P<T> {
            #[inline]
            pub const fn new($($field: T),+) -> $P<T> {
                $P { $($field),+ }
            }
        }

        impl<T: BaseNum> $P<T> {
            #[inline]
            pub fn origin() -> $P<T> {
                $P { $($field: T::zero()),+ }
            }

            #[inline]
            pub fn from_vec(v: $VectorN<T>) -> $P<T> {
                $P::new($(v.$field),+)
            }

            #[inline]
            pub fn to_vec(self) -> $VectorN<T> {
                $VectorN::new($(self.$field),+)
            }

            #[inline]
            pub fn dot(self, v: $VectorN<T>) -> T {
                $VectorN::new($(self.$field * v.$field),+).sum()
            }
        }
    }
}

impl_point!(Point1 { x }, Vec1, 1);
impl_point!(Point2 { x, y }, Vec2, 2);
impl_point!(Point3 { x, y, z }, Vec3, 3);

impl_tuple_conversions!(Point1<S> { x }, (S,));
impl_tuple_conversions!(Point2<S> { x, y }, (S, S,));
impl_tuple_conversions!(Point3<S> { x, y, z }, (S, S, S));

impl_fixed_array_conversions!(Point1<S> { x: 0 }, 1);
impl_fixed_array_conversions!(Point2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Point3<S> { x: 0, y: 1, z: 2 }, 3);

impl<S: fmt::Debug> fmt::Debug for Point1<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point1 ")?;
        <[S; 1] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Point2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point2 ")?;
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Point3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point3 ")?;
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}
