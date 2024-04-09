use super::BoundedRange;

#[derive(Copy, Clone)]
pub struct Brightness(pub u16);

impl BoundedRange<u16> for Brightness {
    const MIN: u16 = 1;
    const MAX: u16 = 100;
}

impl From<u16> for Brightness {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
