use thiserror::Error;

use crate::{Brightness, Percentage, Temperature};

#[derive(Error, Debug)]
pub enum RangeError {
    #[error("Temperature {} not within [{}..{}]", .0, Temperature::MIN, Temperature::MAX)]
    Temperature(u16),
    #[error("Brightness {} not within [{}..{}]", .0, Brightness::MIN, Brightness::MAX)]
    Brightness(u16),
    #[error("Percentage {} not within [{}..{}]", .0, Percentage::MIN, Percentage::MAX)]
    Percentage(i16),
}

pub trait BoundedRange {
    const MIN: i32;
    const MAX: i32;

    fn valid_range(value: i32) -> bool {
        (Self::MIN..=Self::MAX).contains(&value)
    }
}
