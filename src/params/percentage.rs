use super::range_trait::*;

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
