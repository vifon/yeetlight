use std::num::ParseIntError;
use thiserror::Error;

#[derive(Copy, Clone)]
pub enum Effect {
    Smooth(u16),
    Sudden,
}

impl Effect {
    pub fn effect(&self) -> &'static str {
        match self {
            Effect::Smooth(_) => "smooth",
            Effect::Sudden => "sudden",
        }
    }

    pub fn duration(&self) -> u16 {
        match self {
            Effect::Smooth(x) => *x,
            Effect::Sudden => 0,
        }
    }
}

#[derive(Error, Debug)]
pub enum RangeError {
    #[error("Temperature {} not within [{}..{}]", .0, Temperature::MIN, Temperature::MAX)]
    Temperature(u16),
    #[error("Brightness {} not within [{}..{}]", .0, Brightness::MIN, Brightness::MAX)]
    Brightness(u16),
    #[error("Percentage {} not within [{}..{}]", .0, Percentage::MIN, Percentage::MAX)]
    Percentage(i16),
}

trait BoundedRange {
    const MIN: i32;
    const MAX: i32;

    fn valid_range(value: i32) -> bool {
        (Self::MIN..=Self::MAX).contains(&value)
    }
}

#[derive(Copy, Clone)]
pub struct Brightness(pub u16);
impl BoundedRange for Brightness {
    const MIN: i32 = 1;
    const MAX: i32 = 100;
}
impl Brightness {
    pub fn new(brightness: u16) -> Result<Self, RangeError> {
        if Self::valid_range(brightness as i32) {
            Ok(Brightness(brightness))
        } else {
            Err(RangeError::Brightness(brightness))
        }
    }
}

#[derive(Copy, Clone)]
pub struct Temperature(pub u16);
impl BoundedRange for Temperature {
    const MIN: i32 = 1700;
    const MAX: i32 = 6500;
}
impl Temperature {
    pub fn new(temperature: u16) -> Result<Self, RangeError> {
        if Self::valid_range(temperature as i32) {
            Ok(Temperature(temperature))
        } else {
            Err(RangeError::Temperature(temperature))
        }
    }
}

#[derive(Copy, Clone)]
pub struct Color(pub u32);
impl Color {
    pub fn from_hex(hex: &str) -> Result<Color, ParseIntError> {
        Ok(Color(u32::from_str_radix(hex, 16)?))
    }
}

#[derive(Copy, Clone)]
pub struct Percentage(pub i16);
impl BoundedRange for Percentage {
    const MIN: i32 = -100;
    const MAX: i32 = 100;
}
impl Percentage {
    pub fn new(percentage: i16) -> Result<Self, RangeError> {
        if Self::valid_range(percentage as i32) {
            Ok(Percentage(percentage))
        } else {
            Err(RangeError::Percentage(percentage))
        }
    }
}
