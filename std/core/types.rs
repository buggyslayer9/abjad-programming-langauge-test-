//! Core types for the Abjad standard library

/// Represents a value that may or may not be present
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    /// Returns true if the option is a Some value
    pub fn is_some(&self) -> bool {
        matches!(self, Option::Some(_))
    }

    /// Returns true if the option is a None value
    pub fn is_none(&self) -> bool {
        matches!(self, Option::None)
    }

    /// Unwraps the option, yielding the content of a Some
    pub fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!("called unwrap on a None value"),
        }
    }

    /// Unwraps the option, returning the provided default value
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => default,
        }
    }

    /// Maps an Option<T> to Option<U> by applying a function
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(val) => Option::Some(f(val)),
            Option::None => Option::None,
        }
    }

    /// Returns the contained Some value or a default
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Option::Some(val) => val,
            Option::None => T::default(),
        }
    }
}

/// Represents a value that is either success or failure
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    /// Returns true if the result is Ok
    pub fn is_ok(&self) -> bool {
        matches!(self, Result::Ok(_))
    }

    /// Returns true if the result is Err
    pub fn is_err(&self) -> bool {
        matches!(self, Result::Err(_))
    }

    /// Unwraps the result, yielding the content of an Ok
    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(_) => panic!("called unwrap on an Err value"),
        }
    }

    /// Unwraps the result, yielding the content of an Err
    pub fn unwrap_err(self) -> E {
        match self {
            Result::Ok(_) => panic!("called unwrap_err on an Ok value"),
            Result::Err(err) => err,
        }
    }

    /// Maps a Result<T, E> to Result<U, E> by applying a function
    pub fn map<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Result::Ok(val) => Result::Ok(f(val)),
            Result::Err(err) => Result::Err(err),
        }
    }

    /// Maps a Result<T, E> to Result<T, F> by applying a function
    pub fn map_err<F, G>(self, f: F) -> Result<T, F>
    where
        F: FnOnce(E) -> G,
    {
        match self {
            Result::Ok(val) => Result::Ok(val),
            Result::Err(err) => Result::Err(f(err)),
        }
    }
}

/// Represents a generic value that can be any type
#[derive(Debug, Clone, PartialEq)]
pub enum Any {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Any {
    /// Returns true if the value is an integer
    pub fn is_integer(&self) -> bool {
        matches!(self, Any::Integer(_))
    }

    /// Returns true if the value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Any::Float(_))
    }

    /// Returns true if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Any::String(_))
    }

    /// Returns true if the value is a boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, Any::Boolean(_))
    }

    /// Returns true if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Any::Null)
    }
}

/// Represents a range of values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    /// Creates a new range
    pub fn new(start: i64, end: i64) -> Self {
        Range { start, end }
    }

    /// Returns true if the range contains the value
    pub fn contains(&self, value: i64) -> bool {
        value >= self.start && value < self.end
    }

    /// Returns the length of the range
    pub fn len(&self) -> i64 {
        self.end - self.start
    }

    /// Returns true if the range is empty
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// Represents a pair of values
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// Creates a new pair
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

/// Represents a triple of values
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triple<T, U, V> {
    pub first: T,
    pub second: U,
    pub third: V,
}

impl<T, U, V> Triple<T, U, V> {
    /// Creates a new triple
    pub fn new(first: T, second: U, third: V) -> Self {
        Triple {
            first,
            second,
            third,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_some() {
        let opt = Option::Some(42);
        assert!(opt.is_some());
        assert!(!opt.is_none());
        assert_eq!(opt.unwrap(), 42);
    }

    #[test]
    fn test_option_none() {
        let opt: Option<i32> = Option::None;
        assert!(!opt.is_some());
        assert!(opt.is_none());
        assert_eq!(opt.unwrap_or(10), 10);
    }

    #[test]
    fn test_option_map() {
        let opt = Option::Some(5);
        let mapped = opt.map(|x| x * 2);
        assert_eq!(mapped, Option::Some(10));
    }

    #[test]
    fn test_result_ok() {
        let res: Result<i32, &str> = Result::Ok(42);
        assert!(res.is_ok());
        assert!(!res.is_err());
        assert_eq!(res.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let res: Result<i32, &str> = Result::Err("error");
        assert!(!res.is_ok());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "error");
    }

    #[test]
    fn test_result_map() {
        let res = Result::Ok(5);
        let mapped = res.map(|x| x * 2);
        assert_eq!(mapped, Result::Ok(10));
    }

    #[test]
    fn test_any_types() {
        let int_any = Any::Integer(42);
        assert!(int_any.is_integer());
        
        let float_any = Any::Float(3.14);
        assert!(float_any.is_float());
        
        let string_any = Any::String("hello".to_string());
        assert!(string_any.is_string());
        
        let bool_any = Any::Boolean(true);
        assert!(bool_any.is_boolean());
        
        let null_any = Any::Null;
        assert!(null_any.is_null());
    }

    #[test]
    fn test_range() {
        let range = Range::new(0, 10);
        assert!(range.contains(5));
        assert!(!range.contains(10));
        assert_eq!(range.len(), 10);
        assert!(!range.is_empty());
    }

    #[test]
    fn test_pair() {
        let pair = Pair::new(1, "hello");
        assert_eq!(pair.first, 1);
        assert_eq!(pair.second, "hello");
    }

    #[test]
    fn test_triple() {
        let triple = Triple::new(1, "hello", true);
        assert_eq!(triple.first, 1);
        assert_eq!(triple.second, "hello");
        assert_eq!(triple.third, true);
    }
}
