use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Value {} not within [{}..{}]", .value, .min, .max)]
pub struct RangeError<T: Display> {
    value: T,
    min: T,
    max: T,
}

pub trait BoundedRange<T: PartialOrd + Display> {
    const MIN: T;
    const MAX: T;

    fn valid_range(value: &T) -> bool {
        (Self::MIN..=Self::MAX).contains(value)
    }

    fn new(value: T) -> Result<Self, RangeError<T>>
    where
        Self: Sized,
        Self: From<T>,
    {
        if Self::valid_range(&value) {
            Ok(Self::from(value))
        } else {
            Err(RangeError {
                value,
                min: Self::MIN,
                max: Self::MAX,
            })
        }
    }
}
