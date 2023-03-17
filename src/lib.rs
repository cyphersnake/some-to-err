/// The `SomeErr` trait provides a method for converting an `Option` into a `Result` by treating `Some` values as `Err` and `None` values as `Ok`.
///
/// # Examples
///
/// ```
/// use some_to_err::SomeToErr;
///
/// let some: Option<&str> = Some("Error");
/// let result = some.some_to_err(42);
/// assert_eq!(result, Err("Error"));
/// ```
///
/// ```
/// use some_to_err::SomeToErr;
///
/// let none: Option<&str> = None;
/// let result = none.some_to_err(42);
/// assert_eq!(result, Ok(42));
/// ```
pub trait SomeToErr {
    type Err;
    fn some_to_err<OK>(self, ok: OK) -> Result<OK, Self::Err>;
}
impl<T> SomeToErr for Option<T> {
    type Err = T;
    fn some_to_err<OK>(self, ok: OK) -> Result<OK, Self::Err> {
        match self {
            Some(err) => Err(err),
            None => Ok(ok),
        }
    }
}

#[cfg(test)]
mod some_to_err_tests {
    use super::SomeToErr;

    #[test]
    fn test_some_err_some() {
        let some: Option<&str> = Some("Error");
        let result = some.some_to_err(42);
        assert_eq!(result, Err("Error"));
    }

    #[test]
    fn test_some_err_none() {
        let none: Option<&str> = None;
        let result = none.some_to_err(42);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_some_err_some_different_type() {
        let some: Option<i32> = Some(123);
        let result = some.some_to_err("Success");
        assert_eq!(result, Err(123));
    }

    #[test]
    fn test_some_err_none_different_type() {
        let none: Option<i32> = None;
        let result = none.some_to_err("Success");
        assert_eq!(result, Ok("Success"));
    }
}
