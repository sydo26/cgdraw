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

macro_rules! impl_point {
    ($V:ident { $($field:ident),+ }, $n:expr) => {
        impl<S> $V<S> {
            #[inline]
            pub const fn new($($field: S),+) -> $V<S> {
                $V { $($field),+ }
            }
        }
    }
}

impl_point!(Point1 { x }, 1);
impl_point!(Point2 { x, y }, 2);
impl_point!(Point3 { x, y, z }, 3);
