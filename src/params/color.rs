use std::num::ParseIntError;

#[derive(Copy, Clone, Debug)]
pub struct Color(pub u32);

impl Color {
    pub fn from_hex(hex: &str) -> Result<Color, ParseIntError> {
        Ok(Color(u32::from_str_radix(hex, 16)?))
    }
}
