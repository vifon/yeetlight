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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_within_range() {
        let result = Temperature::new(Temperature::MAX - 1);
        assert!(result.is_ok());
        let result = Temperature::new(Temperature::MIN + 1);
        assert!(result.is_ok());
    }

    #[test]
    fn new_out_of_range() {
        let result = Temperature::new(Temperature::MAX + 1);
        assert!(result.is_err());
        let result = Temperature::new(Temperature::MIN - 1);
        assert!(result.is_err());
    }
}
