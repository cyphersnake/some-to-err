#![doc = include_str!("../README.md")]

/// The `SomeErr` trait provides a method for converting an `Option` into a `Result` by treating `Some` values as `Err` and `None` values as `Ok`.
///
/// # Examples
///
/// ```
/// use some_to_err::ErrOr;
///
/// let some: Option<&str> = Some("Error");
/// let result = some.err_or(42);
/// assert_eq!(result, Err("Error"));
/// ```
///
/// ```
/// use some_to_err::ErrOr;
///
/// let none: Option<&str> = None;
/// let result = none.err_or(42);
/// assert_eq!(result, Ok(42));
/// ```
pub trait ErrOr {
    type Err;
    fn err_or<OK>(self, ok: OK) -> Result<OK, Self::Err>;
}
impl<T> ErrOr for Option<T> {
    type Err = T;
    fn err_or<OK>(self, ok: OK) -> Result<OK, Self::Err> {
        match self {
            Some(err) => Err(err),
            None => Ok(ok),
        }
    }
}

#[cfg(test)]
mod err_to_tests {
    use super::ErrOr;

    #[test]
    fn test_some_err_some() {
        let some: Option<&str> = Some("Error");
        let result = some.err_or(42);
        assert_eq!(result, Err("Error"));
    }

    #[test]
    fn test_some_err_none() {
        let none: Option<&str> = None;
        let result = none.err_or(42);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_some_err_some_different_type() {
        let some: Option<i32> = Some(123);
        let result = some.err_or("Success");
        assert_eq!(result, Err(123));
    }

    #[test]
    fn test_some_err_none_different_type() {
        let none: Option<i32> = None;
        let result = none.err_or("Success");
        assert_eq!(result, Ok("Success"));
    }
}

/// The `SomeToErrElse` trait provides a convenient method to convert an `Option<T>` into a `Result<OK, T>`
/// by supplying a closure that generates the `OK` value for the `Result` when the input is `None`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use some_to_err::ErrOrElse;
///
/// let input: Option<&str> = None;
/// let result = input.err_or_else(|| "OK");
/// assert_eq!(result, Ok("OK"));
/// ```
///
/// ```
/// use some_to_err::ErrOrElse;
///
/// let input = Some("Error");
/// let result = input.err_or_else(|| "OK");
/// assert_eq!(result, Err("Error"));
/// ```
pub trait ErrOrElse {
    type Err;
    fn err_or_else<OK>(self, ok: impl FnOnce() -> OK) -> Result<OK, Self::Err>;
}
impl<T> ErrOrElse for Option<T> {
    type Err = T;
    fn err_or_else<OK>(self, ok: impl FnOnce() -> OK) -> Result<OK, Self::Err> {
        match self {
            Some(err) => Err(err),
            None => Ok(ok()),
        }
    }
}

#[cfg(test)]
mod some_to_err_else_tests {
    use super::ErrOrElse;

    #[test]
    fn test_some_to_err_with_some() {
        let input = Some("Error");
        let result = input.err_or_else(|| "OK");
        assert_eq!(result, Err("Error"));
    }

    #[test]
    fn test_some_to_err_with_none() {
        let input: Option<&str> = None;
        let result = input.err_or_else(|| "OK");
        assert_eq!(result, Ok("OK"));
    }

    #[test]
    fn test_some_to_err_with_some_and_closure() {
        let input = Some(2);
        let result = input.err_or_else(|| 3 * 3);
        assert_eq!(result, Err(2));
    }

    #[test]
    fn test_some_to_err_with_none_and_closure() {
        let input: Option<i32> = None;
        let result = input.err_or_else(|| 3 * 3);
        assert_eq!(result, Ok(9));
    }
}
