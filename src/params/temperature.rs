use super::range_trait::*;

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
