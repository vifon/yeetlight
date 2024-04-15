use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Hex string invalid as a color: {}", .0)]
    InvalidColor(String),
    #[error("Unable to parse as hex: {}", .0)]
    Parse(#[from] ParseIntError),
}

#[derive(Copy, Clone, Debug)]
pub struct Color(pub(crate) u32);

impl Color {
    pub fn from_hex(hex: &str) -> Result<Color, ColorError> {
        match hex.len() {
            6 => Ok(Color(u32::from_str_radix(hex, 16)?)),
            _ => Err(ColorError::InvalidColor(hex.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex() {
        let result = Color::from_hex("ff0000");
        assert!(result.is_ok());
        let result = Color::from_hex("FF0000");
        assert!(result.is_ok());
        let result = Color::from_hex("fF0000");
        assert!(result.is_ok());

        // In the future it might be interpretted as "ff0000".
        let result = Color::from_hex("f00");
        assert!(result.is_err());

        let result = Color::from_hex("fff0000");
        assert!(result.is_err());
        let result = Color::from_hex("fff00");
        assert!(result.is_err());
        let result = Color::from_hex("ffoooo");
        assert!(result.is_err());
    }
}
