pub trait BoundedRange<T: PartialOrd> {
    const MIN: T;
    const MAX: T;

    fn valid_range(value: T) -> bool {
        (Self::MIN..=Self::MAX).contains(&value)
    }
}
