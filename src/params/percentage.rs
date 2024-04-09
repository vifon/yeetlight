use super::BoundedRange;

#[derive(Copy, Clone)]
pub struct Percentage(pub i16);

impl BoundedRange<i16> for Percentage {
    const MIN: i16 = -100;
    const MAX: i16 = 100;
}

impl From<i16> for Percentage {
    fn from(value: i16) -> Self {
        Self(value)
    }
}
