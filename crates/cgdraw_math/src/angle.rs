use crate::num::BaseFloat;
use num_traits::cast;
use std::f64;
use std::ops::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Deg<T>(pub T);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Rad<T>(pub T);

impl<T: BaseFloat> Rad<T> {
    #[inline]
    pub fn sin(self) -> T {
        self.0.sin()
    }

    #[inline]
    pub fn cos(self) -> T {
        self.0.cos()
    }

    #[inline]
    pub fn tan(self) -> T {
        self.0.tan()
    }

    #[inline]
    pub fn asin(self) -> T {
        self.0.asin()
    }

    #[inline]
    pub fn acos(self) -> T {
        self.0.acos()
    }

    #[inline]
    pub fn atan(self) -> T {
        self.0.atan()
    }

    #[inline]
    pub fn atan2(self, other: T) -> T {
        self.0.atan2(other)
    }

    #[inline]
    pub fn sin_cos(self) -> (T, T) {
        self.0.sin_cos()
    }

    #[inline]
    pub fn cot(self) -> T {
        self.0.tan().recip()
    }
}

impl<T: BaseFloat> From<Deg<T>> for Rad<T> {
    #[inline]
    fn from(deg: Deg<T>) -> Self {
        Rad(deg.0.to_radians())
    }
}

impl<T: BaseFloat> From<Rad<T>> for Deg<T> {
    #[inline]
    fn from(rad: Rad<T>) -> Deg<T> {
        Deg(rad.0 * cast(180.0 / f64::consts::PI).unwrap())
    }
}

// Rad (*/-%) Rad

impl<T: BaseFloat> Add<Rad<T>> for Rad<T> {
    type Output = Rad<T>;

    #[inline]
    fn add(self, other: Rad<T>) -> Rad<T> {
        Rad(self.0 + other.0)
    }
}

impl<T: BaseFloat> Sub<Rad<T>> for Rad<T> {
    type Output = Rad<T>;

    #[inline]
    fn sub(self, other: Rad<T>) -> Rad<T> {
        Rad(self.0 - other.0)
    }
}

impl<T: BaseFloat> Div<Rad<T>> for Rad<T> {
    type Output = T;

    #[inline]
    fn div(self, other: Rad<T>) -> T {
        self.0 / other.0
    }
}

impl<T: BaseFloat> Rem<Rad<T>> for Rad<T> {
    type Output = Rad<T>;

    #[inline]
    fn rem(self, other: Rad<T>) -> Rad<T> {
        Rad(self.0 % other.0)
    }
}

// Rad (*/-%) Scalar

impl<T: BaseFloat> Add<T> for Rad<T> {
    type Output = Rad<T>;

    #[inline]
    fn add(self, scalar: T) -> Rad<T> {
        Rad(self.0 + scalar)
    }
}

impl<T: BaseFloat> Sub<T> for Rad<T> {
    type Output = Rad<T>;

    #[inline]
    fn sub(self, scalar: T) -> Rad<T> {
        Rad(self.0 - scalar)
    }
}

impl<T: BaseFloat> Div<T> for Rad<T> {
    type Output = Rad<T>;

    #[inline]
    fn div(self, scalar: T) -> Rad<T> {
        Rad(self.0 / scalar)
    }
}

impl<T: BaseFloat> Rem<T> for Rad<T> {
    type Output = Rad<T>;

    #[inline]
    fn rem(self, scalar: T) -> Rad<T> {
        Rad(self.0 % scalar)
    }
}
