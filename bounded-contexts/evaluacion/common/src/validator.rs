use thiserror::Error;

/// Error types for validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidatorError {
    #[error("Invalid regex pattern: {0}")]
    RegexError(String),

    #[error("Value cannot be empty")]
    EmptyValue,

    #[error("Length must be at least {min} characters, but got {actual}")]
    MinLength { min: usize, actual: usize },

    #[error("Length must be the at most {max} characters, but got {actual}")]
    MaxLength { max: usize, actual: usize },

    #[error("Length must be between {min} and {max} characters, but got {actual}")]
    LengthOutOfRange {
        min: usize,
        max: usize,
        actual: usize,
    },
}

/// Service for common string validation operations.
///
/// Provides reusable validation logic for strings, including length checks,
/// emptiness validation, and character validation.
pub struct Validator;

impl Validator {
    /// Validates that a string is not empty after trimming whitespace.
    ///
    /// # Errors
    ///
    /// Returns `ValidatorError::EmptyValue` if the string is empty or contains only whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use common::Validator;
    ///
    /// assert!(Validator::is_not_empty("hello").is_ok());
    /// assert!(Validator::is_not_empty("  hello  ").is_ok());
    /// assert!(Validator::is_not_empty("").is_err());
    /// assert!(Validator::is_not_empty("   ").is_err());
    /// ```
    pub fn is_not_empty(value: &str) -> Result<(), ValidatorError> {
        if value.trim().is_empty() {
            return Err(ValidatorError::EmptyValue);
        }
        Ok(())
    }

    /// Validates that a string has at least a minimum length.
    ///
    /// # Errors
    ///
    /// Returns `ValidatorError::MinLength` if the string length is less than the minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use common::Validator;
    ///
    /// assert!(Validator::has_min_length("hello", 3).is_ok());
    /// assert!(Validator::has_min_length("hello", 5).is_ok());
    /// assert!(Validator::has_min_length("hi", 3).is_err());
    /// ```
    pub fn has_min_length(value: &str, min: usize) -> Result<(), ValidatorError> {
        let actual = value.len();
        if actual < min {
            return Err(ValidatorError::MinLength { min, actual });
        }
        Ok(())
    }

    /// Validates that a string does not exceed a maximum length.
    ///
    /// # Errors
    ///
    /// Returns `ValidatorError::MaxLength` if the string length exceeds the maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use common::Validator;
    ///
    /// assert!(Validator::has_max_length("hello", 10).is_ok());
    /// assert!(Validator::has_max_length("hello", 5).is_ok());
    /// assert!(Validator::has_max_length("hello world", 5).is_err());
    /// ```
    pub fn has_max_length(value: &str, max: usize) -> Result<(), ValidatorError> {
        let actual = value.len();
        if actual > max {
            return Err(ValidatorError::MaxLength { max, actual });
        }
        Ok(())
    }

    /// Validates that a string length is within a specified range (inclusive).
    ///
    /// # Errors
    ///
    /// Returns `ValidatorError::LengthOutOfRange` if the string length is outside the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use common::Validator;
    ///
    /// assert!(Validator::has_length_between("hello", 3, 10).is_ok());
    /// assert!(Validator::has_length_between("hello", 5, 5).is_ok());
    /// assert!(Validator::has_length_between("hi", 3, 10).is_err());
    /// assert!(Validator::has_length_between("hello world", 3, 5).is_err());
    /// ```
    pub fn has_length_between(value: &str, min: usize, max: usize) -> Result<(), ValidatorError> {
        let actual = value.len();
        if actual < min || actual > max {
            return Err(ValidatorError::LengthOutOfRange { min, max, actual });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for is_not_empty
    #[test]
    fn test_is_not_empty_with_valid_string() {
        assert!(Validator::is_not_empty("hello").is_ok());
    }

    #[test]
    fn test_is_not_empty_with_whitespace_around() {
        assert!(Validator::is_not_empty("  hello  ").is_ok());
    }

    #[test]
    fn test_is_not_empty_with_empty_string() {
        let result = Validator::is_not_empty("");
        assert!(matches!(result, Err(ValidatorError::EmptyValue)));
    }

    #[test]
    fn test_is_not_empty_with_only_whitespace() {
        let result = Validator::is_not_empty("   ");
        assert!(matches!(result, Err(ValidatorError::EmptyValue)));
    }

    #[test]
    fn test_is_not_empty_with_tabs_and_newlines() {
        let result = Validator::is_not_empty("\t\n  ");
        assert!(matches!(result, Err(ValidatorError::EmptyValue)));
    }

    // Tests for has_min_length
    #[test]
    fn test_has_min_length_exact_minimum() {
        assert!(Validator::has_min_length("hello", 5).is_ok());
    }

    #[test]
    fn test_has_min_length_above_minimum() {
        assert!(Validator::has_min_length("hello world", 5).is_ok());
    }

    #[test]
    fn test_has_min_length_below_minimum() {
        let result = Validator::has_min_length("hi", 3);
        assert!(matches!(
            result,
            Err(ValidatorError::MinLength { min: 3, actual: 2 })
        ));
    }

    #[test]
    fn test_has_min_length_zero() {
        assert!(Validator::has_min_length("", 0).is_ok());
        assert!(Validator::has_min_length("hello", 0).is_ok());
    }

    #[test]
    fn test_has_min_length_with_unicode() {
        // "José" is 5 bytes but 4 characters
        assert!(Validator::has_min_length("José", 4).is_ok());
    }

    // Tests for has_max_length
    #[test]
    fn test_has_max_length_exact_maximum() {
        assert!(Validator::has_max_length("hello", 5).is_ok());
    }

    #[test]
    fn test_has_max_length_below_maximum() {
        assert!(Validator::has_max_length("hi", 5).is_ok());
    }

    #[test]
    fn test_has_max_length_above_maximum() {
        let result = Validator::has_max_length("hello world", 5);
        assert!(matches!(
            result,
            Err(ValidatorError::MaxLength { max: 5, actual: 11 })
        ));
    }

    #[test]
    fn test_has_max_length_empty_string() {
        assert!(Validator::has_max_length("", 10).is_ok());
    }

    // Tests for has_length_between
    #[test]
    fn test_has_length_between_within_range() {
        assert!(Validator::has_length_between("hello", 3, 10).is_ok());
    }

    #[test]
    fn test_has_length_between_at_minimum() {
        assert!(Validator::has_length_between("hello", 5, 10).is_ok());
    }

    #[test]
    fn test_has_length_between_at_maximum() {
        assert!(Validator::has_length_between("hello", 3, 5).is_ok());
    }

    #[test]
    fn test_has_length_between_exact_range() {
        assert!(Validator::has_length_between("hello", 5, 5).is_ok());
    }

    #[test]
    fn test_has_length_between_below_minimum() {
        let result = Validator::has_length_between("hi", 3, 10);
        assert!(matches!(
            result,
            Err(ValidatorError::LengthOutOfRange {
                min: 3,
                max: 10,
                actual: 2
            })
        ));
    }

    #[test]
    fn test_has_length_between_above_maximum() {
        let result = Validator::has_length_between("hello world", 3, 5);
        assert!(matches!(
            result,
            Err(ValidatorError::LengthOutOfRange {
                min: 3,
                max: 5,
                actual: 11
            })
        ));
    }

    #[test]
    fn test_error_provides_actual_length() {
        let result = Validator::has_min_length("ab", 5);
        match result {
            Err(ValidatorError::MinLength { min, actual }) => {
                assert_eq!(min, 5);
                assert_eq!(actual, 2);
            }
            _ => panic!("Expected MinLength error"),
        }
    }
}
