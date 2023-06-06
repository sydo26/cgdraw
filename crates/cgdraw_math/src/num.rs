use std::fmt;
use std::ops::*;

use num_traits::*;

pub trait BaseNum:
    Copy
    + Clone
    + fmt::Debug
    + Num
    + NumCast
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
}

impl<T> BaseNum for T where
    T: Copy
        + Clone
        + fmt::Debug
        + Num
        + NumCast
        + PartialOrd
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
{
}

/// Base floating point types
pub trait BaseFloat:
    BaseNum
    + Float
    + approx::AbsDiffEq<Epsilon = Self>
    + approx::RelativeEq<Epsilon = Self>
    + approx::UlpsEq<Epsilon = Self>
{
}

impl<T> BaseFloat for T where
    T: BaseNum
        + num_traits::Float
        + approx::AbsDiffEq<Epsilon = Self>
        + approx::RelativeEq<Epsilon = Self>
        + approx::UlpsEq<Epsilon = Self>
{
}
