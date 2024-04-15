use super::BoundedRange;

#[derive(Copy, Clone, Debug)]
pub struct Brightness(pub(crate) u16);

impl BoundedRange<u16> for Brightness {
    const MIN: u16 = 1;
    const MAX: u16 = 100;
}

impl From<u16> for Brightness {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_within_range() {
        let result = Brightness::new(Brightness::MAX - 1);
        assert!(result.is_ok());
        let result = Brightness::new(Brightness::MIN + 1);
        assert!(result.is_ok());
    }

    #[test]
    fn new_out_of_range() {
        let result = Brightness::new(Brightness::MAX + 1);
        assert!(result.is_err());
        let result = Brightness::new(Brightness::MIN - 1);
        assert!(result.is_err());
    }
}
