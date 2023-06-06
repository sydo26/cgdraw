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
        impl<S> $V<S> {
            #[inline]
            pub const fn new($($field: S),+) -> $V<S> {
                $V { $($field),+ }
            }
        }
    }
}

impl_vector!(Vec1 { x }, 1);
impl_vector!(Vec2 { x, y }, 2);
impl_vector!(Vec3 { x, y, z }, 3);
impl_vector!(Vec4 { x, y, z, w }, 4);
