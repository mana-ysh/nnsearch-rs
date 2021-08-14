use num::{Float};
use std::fmt::{Debug, Display};

pub trait FloatScalar: Float + Debug + Display {
    fn get_zero() -> Self;
}

impl FloatScalar for f32 {
    fn get_zero() -> Self {
        0.0
    }
}

pub trait HashedScaler: Debug + Display {}

impl HashedScaler for u32 {}
impl HashedScaler for i32 {}