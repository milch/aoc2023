pub trait OptionFlatMap<T, U, F: FnOnce(T) -> Option<U>> {
    fn flat_map(self, f: F) -> Option<U>;
}

impl<T, U, F> OptionFlatMap<T, U, F> for Option<T>
where
    F: FnOnce(T) -> Option<U>,
{
    fn flat_map(self, f: F) -> Option<U> {
        match self {
            Some(x) => f(x),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_flat_map() {
        assert_eq!(Some(5).flat_map(|x| Some(x * 2)), Some(10));
        let none_option: Option<i32> = None;
        assert_eq!(none_option.flat_map(|x| Some(x * 2)), None);
    }
}
