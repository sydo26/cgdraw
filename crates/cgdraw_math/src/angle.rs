use crate::num::BaseFloat;
use num_traits::cast;
use std::f64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Deg<T>(pub T);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Rad<S>(pub S);

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
