use super::range_trait::*;

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
