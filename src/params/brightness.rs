use thiserror::Error;

use super::range_trait::BoundedRange;

#[derive(Error, Debug)]
#[error("Brightness {} not within [{}..{}]", .0, Brightness::MIN, Brightness::MAX)]
pub struct BrightnessRangeError(u16);

#[derive(Copy, Clone)]
pub struct Brightness(pub u16);
impl BoundedRange<u16> for Brightness {
    const MIN: u16 = 1;
    const MAX: u16 = 100;
}
impl Brightness {
    pub fn new(brightness: u16) -> Result<Self, BrightnessRangeError> {
        if Self::valid_range(brightness) {
            Ok(Brightness(brightness))
        } else {
            Err(BrightnessRangeError(brightness))
        }
    }
}
