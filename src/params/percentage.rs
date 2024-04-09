use thiserror::Error;

use super::range_trait::BoundedRange;

#[derive(Error, Debug)]
#[error("Percentage {} not within [{}..{}]", .0, Percentage::MIN, Percentage::MAX)]
pub struct PercentageRangeError(i16);

#[derive(Copy, Clone)]
pub struct Percentage(pub i16);
impl BoundedRange<i16> for Percentage {
    const MIN: i16 = -100;
    const MAX: i16 = 100;
}
impl Percentage {
    pub fn new(percentage: i16) -> Result<Self, PercentageRangeError> {
        if Self::valid_range(percentage) {
            Ok(Percentage(percentage))
        } else {
            Err(PercentageRangeError(percentage))
        }
    }
}
