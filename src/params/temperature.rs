use super::BoundedRange;

#[derive(Copy, Clone, Debug)]
pub struct Temperature(pub u16);

impl BoundedRange<u16> for Temperature {
    const MIN: u16 = 1700;
    const MAX: u16 = 6500;
}

impl From<u16> for Temperature {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
