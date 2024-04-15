use super::BoundedRange;

#[derive(Copy, Clone, Debug)]
pub struct Percentage(pub(crate) i16);

impl BoundedRange<i16> for Percentage {
    const MIN: i16 = -100;
    const MAX: i16 = 100;
}

impl From<i16> for Percentage {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_within_range() {
        let result = Percentage::new(Percentage::MAX - 1);
        assert!(result.is_ok());
        let result = Percentage::new(Percentage::MIN + 1);
        assert!(result.is_ok());
    }

    #[test]
    fn new_out_of_range() {
        let result = Percentage::new(Percentage::MAX + 1);
        assert!(result.is_err());
        let result = Percentage::new(Percentage::MIN - 1);
        assert!(result.is_err());
    }
}
