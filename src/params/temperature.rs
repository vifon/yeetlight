use thiserror::Error;

use super::range_trait::BoundedRange;

#[derive(Error, Debug)]
#[error("Temperature {} not within [{}..{}]", .0, Temperature::MIN, Temperature::MAX)]
pub struct TemperatureRangeError(u16);

#[derive(Copy, Clone)]
pub struct Temperature(pub u16);
impl BoundedRange<u16> for Temperature {
    const MIN: u16 = 1700;
    const MAX: u16 = 6500;
}
impl Temperature {
    pub fn new(temperature: u16) -> Result<Self, TemperatureRangeError> {
        if Self::valid_range(temperature) {
            Ok(Temperature(temperature))
        } else {
            Err(TemperatureRangeError(temperature))
        }
    }
}
