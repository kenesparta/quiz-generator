use crate::{Validator, ValidatorError};
use std::fmt;
use std::ops::Deref;
use thiserror::Error;

const DEFAULT_MAX_LENGTH: usize = 100;
const DEFAULT_MIN_LENGTH: usize = 1;

/// Error type for `SimpleName` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SimpleNameError {
    /// Validation error occurred during name validation.
    #[error("SimpleName validation failed: {0}")]
    ValidationError(#[from] ValidatorError),

    #[error("SimpleName is empty")]
    EmptyValue,

    #[error("SimpleName contains invalid characters")]
    CharactersNotValid,
}

/// Configuration for simple name validation rules.
///
/// # Examples
///
/// ```
/// use education_platform_common::SimpleNameConfig;
///
/// let config = SimpleNameConfig::default();
/// assert_eq!(config.min_length(), 1);
/// assert_eq!(config.max_length(), 100);
///
/// let custom = SimpleNameConfig::builder()
///     .min_length(3)
///     .max_length(50)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SimpleNameConfig {
    min_length: usize,
    max_length: usize,
}

impl SimpleNameConfig {
    /// Creates a new `SimpleNameConfig` with the specified constraints.
    ///
    /// # Panics
    ///
    /// Panics if `min_length` is greater than `max_length`.
    #[must_use]
    pub const fn new(min_length: usize, max_length: usize) -> Self {
        assert!(
            min_length <= max_length,
            "min_length must be less than or equal to max_length"
        );
        Self {
            min_length,
            max_length,
        }
    }

    /// Creates a builder for `SimpleNameConfig`.
    #[must_use]
    pub const fn builder() -> SimpleNameConfigBuilder {
        SimpleNameConfigBuilder::new()
    }

    /// Returns the minimum allowed length.
    #[inline]
    #[must_use]
    pub const fn min_length(&self) -> usize {
        self.min_length
    }

    /// Returns the maximum allowed length.
    #[inline]
    #[must_use]
    pub const fn max_length(&self) -> usize {
        self.max_length
    }
}

impl Default for SimpleNameConfig {
    /// Creates a default configuration with min_length=1 and max_length=100.
    fn default() -> Self {
        Self::new(DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH)
    }
}

/// Builder for `SimpleNameConfig`.
#[derive(Debug, Clone, Copy)]
pub struct SimpleNameConfigBuilder {
    min_length: usize,
    max_length: usize,
}

impl SimpleNameConfigBuilder {
    /// Creates a new builder with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            min_length: DEFAULT_MIN_LENGTH,
            max_length: DEFAULT_MAX_LENGTH,
        }
    }

    /// Sets the minimum length constraint.
    #[must_use]
    pub const fn min_length(mut self, min: usize) -> Self {
        self.min_length = min;
        self
    }

    /// Sets the maximum length constraint.
    #[must_use]
    pub const fn max_length(mut self, max: usize) -> Self {
        self.max_length = max;
        self
    }

    /// Builds the `SimpleNameConfig`.
    ///
    /// # Panics
    ///
    /// Panics if `min_length` is greater than `max_length`.
    #[must_use]
    pub const fn build(self) -> SimpleNameConfig {
        SimpleNameConfig::new(self.min_length, self.max_length)
    }
}

impl Default for SimpleNameConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A validated simple name for courses, blog titles, and descriptive names.
///
/// `SimpleName` is a Value Object for names that allow rich text with
/// alphanumeric characters, spaces, and common special characters. Suitable for:
/// - Course names: "Introduction to Rust Programming!"
/// - Blog titles: "My 'awesome' Journey @ 2024"
/// - Descriptive names: "Web Dev 101: Learn HTML & CSS"
///
/// # Allowed Characters
///
/// - Alphabetic: a-z, A-Z (including diacritics like á, é, ñ)
/// - Numeric: 0-9
/// - Spaces (allowed)
/// - Special: @ - _ / . : ' " ! # $ % & ( ) + , ;
/// - **NOT allowed**: control characters (tabs, newlines), backslash, pipes
///
/// # Examples
///
/// ```
/// use education_platform_common::SimpleName;
///
/// // Course names with spaces and punctuation
/// let course = SimpleName::new("This is 'my awesome' course!".to_string()).unwrap();
/// assert_eq!(course.as_str(), "This is 'my awesome' course!");
///
/// // Blog titles with special chars
/// let blog = SimpleName::new("My Course is Really good 4 all, @my_book".to_string()).unwrap();
/// assert_eq!(blog.as_str(), "My Course is Really good 4 all, @my_book");
///
/// // With quotes and numbers
/// let title = SimpleName::new("\"Best Course\" of 2024!".to_string()).unwrap();
/// assert_eq!(title.as_str(), "\"Best Course\" of 2024!");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SimpleName {
    inner: String,
    config: SimpleNameConfig,
}

impl SimpleName {
    /// Creates a new `SimpleName` with default validation rules.
    ///
    /// The name will be trimmed and validated to:
    /// - Not be empty after trimming
    /// - Have length >= 1 character
    /// - Have length <= 100 characters
    /// - Contain only valid characters (alphanumeric, spaces, common punctuation)
    /// - Not contain control characters (tabs, newlines)
    ///
    /// # Errors
    ///
    /// Returns `SimpleNameError::ValidationError` if validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::SimpleName;
    ///
    /// let name = SimpleName::new("Introduction to Rust".to_string()).unwrap();
    /// assert_eq!(name.as_str(), "Introduction to Rust");
    ///
    /// // Whitespace is trimmed
    /// let trimmed = SimpleName::new("  My Course  ".to_string()).unwrap();
    /// assert_eq!(trimmed.as_str(), "My Course");
    ///
    /// // Special characters allowed
    /// let with_special = SimpleName::new("Web Dev 101: The 'Best' Course!".to_string()).unwrap();
    /// assert_eq!(with_special.as_str(), "Web Dev 101: The 'Best' Course!");
    /// ```
    pub fn new(name: String) -> Result<Self, SimpleNameError> {
        Self::with_config(name, SimpleNameConfig::default())
    }

    /// Creates a new `SimpleName` with custom validation configuration.
    ///
    /// # Errors
    ///
    /// Returns `SimpleNameError::ValidationError` if validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::{SimpleName, SimpleNameConfig};
    ///
    /// let config = SimpleNameConfig::builder()
    ///     .min_length(5)
    ///     .max_length(20)
    ///     .build();
    ///
    /// let name = SimpleName::with_config("course-101".to_string(), config).unwrap();
    /// assert_eq!(name.as_str(), "course-101");
    ///
    /// // Too short for this config
    /// let result = SimpleName::with_config("api".to_string(), config);
    /// assert!(result.is_err());
    /// ```
    pub fn with_config(name: String, config: SimpleNameConfig) -> Result<Self, SimpleNameError> {
        let trimmed = name.trim();

        Validator::is_not_empty(trimmed)?;
        Validator::has_min_length(trimmed, config.min_length)?;
        Validator::has_max_length(trimmed, config.max_length)?;
        Self::is_valid_simple_name(trimmed)?;

        Ok(Self {
            inner: trimmed.to_string(),
            config,
        })
    }

    /// Validates that a name contains only allowed characters.
    ///
    /// Allowed characters:
    /// - Alphabetic (including diacritics): a-z, A-Z, á, é, ñ, etc.
    /// - Numeric: 0-9
    /// - Spaces (single spaces allowed)
    /// - Special: @ - _ / . : ' " ! # $ % & ( ) + , ; = ? * [ ]
    ///
    /// Not allowed (for security and formatting):
    /// - Control characters (tabs, newlines, etc.)
    /// - Backslash: \
    /// - Pipe: |
    /// - Braces: { }
    /// - Angle brackets: < >
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::SimpleName;
    ///
    /// // Valid course/blog names
    /// assert!(SimpleName::is_valid_simple_name("This is 'my awesome' course!").is_ok());
    /// assert!(SimpleName::is_valid_simple_name("Web Dev 101").is_ok());
    /// assert!(SimpleName::is_valid_simple_name("\"Best Course\" of 2024!").is_ok());
    /// assert!(SimpleName::is_valid_simple_name("My Course @ Home").is_ok());
    /// assert!(SimpleName::is_valid_simple_name("Course #1: Introduction").is_ok());
    /// assert!(SimpleName::is_valid_simple_name("50% Off Sale!").is_ok());
    ///
    /// // Invalid names
    /// assert!(SimpleName::is_valid_simple_name("test\tname").is_err());   // tab
    /// assert!(SimpleName::is_valid_simple_name("test\nname").is_err());   // newline
    /// assert!(SimpleName::is_valid_simple_name("test\\name").is_err());   // backslash
    /// assert!(SimpleName::is_valid_simple_name("test|name").is_err());    // pipe
    /// assert!(SimpleName::is_valid_simple_name("test<name>").is_err());   // angle brackets
    /// ```
    pub fn is_valid_simple_name(name: &str) -> Result<(), SimpleNameError> {
        if name.is_empty() {
            return Err(SimpleNameError::EmptyValue);
        }

        // Only exclude control characters and a few unsafe/problematic chars
        let has_invalid_char = name
            .chars()
            .any(|c| c.is_control() || matches!(c, '\\' | '|' | '{' | '}' | '<' | '>'));

        if has_invalid_char {
            return Err(SimpleNameError::CharactersNotValid);
        }

        Ok(())
    }

    /// Returns the name as a string slice.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Returns the configuration used for this name.
    #[inline]
    #[must_use]
    pub const fn config(&self) -> &SimpleNameConfig {
        &self.config
    }

    /// Consumes the `SimpleName` and returns the inner `String`.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl Deref for SimpleName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for SimpleName {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl fmt::Display for SimpleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<SimpleName> for String {
    fn from(name: SimpleName) -> Self {
        name.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_with_valid_simple_name() {
            let result = SimpleName::new("course-101".to_string());
            assert!(result.is_ok());
            assert_eq!(result.unwrap().as_str(), "course-101");
        }

        #[test]
        fn test_new_trims_whitespace() {
            let result = SimpleName::new("  api-v1  ".to_string());
            assert!(result.is_ok());
            assert_eq!(result.unwrap().as_str(), "api-v1");
        }

        #[test]
        fn test_new_with_empty_string_returns_error() {
            let result = SimpleName::new("".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn test_new_with_whitespace_only_returns_error() {
            let result = SimpleName::new("   ".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn test_with_config_custom_min_max() {
            let config = SimpleNameConfig::builder()
                .min_length(5)
                .max_length(20)
                .build();

            let result = SimpleName::with_config("course-101".to_string(), config);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().as_str(), "course-101");
        }

        #[test]
        fn test_with_config_too_short_returns_error() {
            let config = SimpleNameConfig::builder()
                .min_length(10)
                .max_length(50)
                .build();

            let result = SimpleName::with_config("api".to_string(), config);
            assert!(result.is_err());
        }

        #[test]
        fn test_with_config_too_long_returns_error() {
            let config = SimpleNameConfig::builder()
                .min_length(1)
                .max_length(5)
                .build();

            let result = SimpleName::with_config("very-long-name".to_string(), config);
            assert!(result.is_err());
        }

        #[test]
        fn test_default_config_values() {
            let config = SimpleNameConfig::default();
            assert_eq!(config.min_length(), 1);
            assert_eq!(config.max_length(), 100);
        }

        #[test]
        fn test_config_builder() {
            let config = SimpleNameConfig::builder()
                .min_length(3)
                .max_length(50)
                .build();
            assert_eq!(config.min_length(), 3);
            assert_eq!(config.max_length(), 50);
        }
    }

    mod character_validation {
        use super::*;

        #[test]
        fn test_alphanumeric_characters() {
            assert!(SimpleName::new("abc123".to_string()).is_ok());
            assert!(SimpleName::new("ABC123".to_string()).is_ok());
            assert!(SimpleName::new("test123TEST".to_string()).is_ok());
        }

        #[test]
        fn test_spaces_allowed() {
            assert!(SimpleName::new("hello world".to_string()).is_ok());
            assert!(SimpleName::new("My Awesome Course".to_string()).is_ok());
            assert!(SimpleName::new("Course 101".to_string()).is_ok());
        }

        #[test]
        fn test_special_characters_allowed() {
            assert!(SimpleName::new("user@domain".to_string()).is_ok());
            assert!(SimpleName::new("api-v1".to_string()).is_ok());
            assert!(SimpleName::new("file_name".to_string()).is_ok());
            assert!(SimpleName::new("path/to/file".to_string()).is_ok());
            assert!(SimpleName::new("version.1.0".to_string()).is_ok());
            assert!(SimpleName::new("api:v2".to_string()).is_ok());
            assert!(SimpleName::new("test!name".to_string()).is_ok());
            assert!(SimpleName::new("name$123".to_string()).is_ok());
            assert!(SimpleName::new("user%name".to_string()).is_ok());
            assert!(SimpleName::new("test&name".to_string()).is_ok());
            assert!(SimpleName::new("name*123".to_string()).is_ok());
            assert!(SimpleName::new("test(name)".to_string()).is_ok());
            assert!(SimpleName::new("name[123]".to_string()).is_ok());
            assert!(SimpleName::new("name;123".to_string()).is_ok());
            assert!(SimpleName::new("test'name".to_string()).is_ok());
            assert!(SimpleName::new("name\"123".to_string()).is_ok());
            assert!(SimpleName::new("name,123".to_string()).is_ok());
            assert!(SimpleName::new("test?name".to_string()).is_ok());
        }

        #[test]
        fn test_quotes_and_punctuation() {
            assert!(SimpleName::new("This is 'my awesome' course!".to_string()).is_ok());
            assert!(SimpleName::new("\"Best Course\" of 2024!".to_string()).is_ok());
            assert!(SimpleName::new("Course #1: Introduction".to_string()).is_ok());
            assert!(SimpleName::new("50% Off Sale!".to_string()).is_ok());
        }

        #[test]
        fn test_diacritics_allowed() {
            assert!(SimpleName::new("José 123".to_string()).is_ok());
            assert!(SimpleName::new("María García".to_string()).is_ok());
            assert!(SimpleName::new("João Silva".to_string()).is_ok());
            assert!(SimpleName::new("Curso de Español @ 2024".to_string()).is_ok());
        }

        #[test]
        fn test_invalid_control_characters() {
            assert!(SimpleName::new("test\tname".to_string()).is_err());
            assert!(SimpleName::new("test\nname".to_string()).is_err());
            assert!(SimpleName::new("test\r\nname".to_string()).is_err());
        }

        #[test]
        fn test_invalid_unsafe_characters() {
            assert!(SimpleName::new("test\\name".to_string()).is_err()); // backslash
            assert!(SimpleName::new("test|name".to_string()).is_err()); // pipe
            assert!(SimpleName::new("test{name}".to_string()).is_err()); // braces
            assert!(SimpleName::new("test<name>".to_string()).is_err()); // angle brackets
        }
    }

    mod real_world_examples {
        use super::*;

        #[test]
        fn test_course_names() {
            assert!(SimpleName::new("Introduction to Rust Programming".to_string()).is_ok());
            assert!(SimpleName::new("Web Development 101: HTML & CSS".to_string()).is_ok());
            assert!(SimpleName::new("Advanced JavaScript (ES6+)".to_string()).is_ok());
            assert!(SimpleName::new("Python Fundamentals".to_string()).is_ok());
            assert!(SimpleName::new("Data Science with R & Python".to_string()).is_ok());
        }

        #[test]
        fn test_blog_titles() {
            assert!(SimpleName::new("This is 'my awesome' course!".to_string()).is_ok());
            assert!(
                SimpleName::new("My Course is Really good 4 all, @my_book".to_string()).is_ok()
            );
            assert!(SimpleName::new("\"Best Practices\" for Clean Code".to_string()).is_ok());
            assert!(SimpleName::new("10 Tips & Tricks for Success!".to_string()).is_ok());
        }

        #[test]
        fn test_event_names() {
            assert!(SimpleName::new("Annual Conference 2024".to_string()).is_ok());
            assert!(SimpleName::new("Workshop: Git & GitHub Basics".to_string()).is_ok());
            assert!(SimpleName::new("Webinar - Cloud Computing 101".to_string()).is_ok());
        }

        #[test]
        fn test_with_special_punctuation() {
            assert!(SimpleName::new("Course #1: Getting Started".to_string()).is_ok());
            assert!(SimpleName::new("50% Discount - Limited Time!".to_string()).is_ok());
            assert!(SimpleName::new("Q&A Session (Live)".to_string()).is_ok());
            assert!(SimpleName::new("Project: Build a Todo App".to_string()).is_ok());
        }

        #[test]
        fn test_multilingual_names() {
            assert!(SimpleName::new("Curso de Español para Principiantes".to_string()).is_ok());
            assert!(SimpleName::new("Português: Gramática & Vocabulário".to_string()).is_ok());
            assert!(SimpleName::new("José's Guide to Success".to_string()).is_ok());
        }
    }

    mod value_object_semantics {
        use super::*;

        #[test]
        fn test_equality() {
            let name1 = SimpleName::new("test-name".to_string()).unwrap();
            let name2 = SimpleName::new("test-name".to_string()).unwrap();
            assert_eq!(name1, name2);
        }

        #[test]
        fn test_inequality() {
            let name1 = SimpleName::new("test-1".to_string()).unwrap();
            let name2 = SimpleName::new("test-2".to_string()).unwrap();
            assert_ne!(name1, name2);
        }

        #[test]
        fn test_clone() {
            let name1 = SimpleName::new("original".to_string()).unwrap();
            let name2 = name1.clone();
            assert_eq!(name1, name2);
        }

        #[test]
        fn test_hash_consistency() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            let name1 = SimpleName::new("test".to_string()).unwrap();
            let name2 = SimpleName::new("test".to_string()).unwrap();
            set.insert(name1);
            assert!(set.contains(&name2));
        }

        #[test]
        fn test_ordering() {
            let name1 = SimpleName::new("aaa".to_string()).unwrap();
            let name2 = SimpleName::new("bbb".to_string()).unwrap();
            assert!(name1 < name2);
            assert!(name2 > name1);
        }

        #[test]
        fn test_debug_format() {
            let name = SimpleName::new("test".to_string()).unwrap();
            let debug = format!("{:?}", name);
            assert!(debug.contains("SimpleName"));
        }

        #[test]
        fn test_display_format() {
            let name = SimpleName::new("display-test".to_string()).unwrap();
            assert_eq!(format!("{}", name), "display-test");
        }

        #[test]
        fn test_deref() {
            let name = SimpleName::new("test-123".to_string()).unwrap();
            assert_eq!(name.len(), 8);
            assert!(name.starts_with("test"));
        }

        #[test]
        fn test_as_ref() {
            let name = SimpleName::new("reference".to_string()).unwrap();
            let s: &str = name.as_ref();
            assert_eq!(s, "reference");
        }

        #[test]
        fn test_into_string() {
            let name = SimpleName::new("convert".to_string()).unwrap();
            let string: String = name.into();
            assert_eq!(string, "convert");
        }

        #[test]
        fn test_into_inner() {
            let name = SimpleName::new("inner".to_string()).unwrap();
            let string = name.into_inner();
            assert_eq!(string, "inner");
        }
    }

    mod boundary_tests {
        use super::*;

        #[test]
        fn test_single_character() {
            let name = SimpleName::new("a".to_string()).unwrap();
            assert_eq!(name.as_str(), "a");
        }

        #[test]
        fn test_at_max_length() {
            let long_name = "a".repeat(100);
            let name = SimpleName::new(long_name.clone()).unwrap();
            assert_eq!(name.as_str(), &long_name);
        }

        #[test]
        fn test_exceeds_max_length() {
            let too_long = "a".repeat(101);
            let result = SimpleName::new(too_long);
            assert!(result.is_err());
        }

        #[test]
        fn test_all_allowed_characters() {
            let all_chars = "abcXYZ123@-_/.:";
            let name = SimpleName::new(all_chars.to_string()).unwrap();
            assert_eq!(name.as_str(), all_chars);
        }
    }

    mod error_handling {
        use super::*;

        #[test]
        fn test_empty_error() {
            let result = SimpleName::is_valid_simple_name("");
            assert!(matches!(result, Err(SimpleNameError::EmptyValue)));
        }

        #[test]
        fn test_invalid_characters_error() {
            let result = SimpleName::is_valid_simple_name("test|name");
            assert!(matches!(result, Err(SimpleNameError::CharactersNotValid)));
        }

        #[test]
        fn test_validation_error_from_validator() {
            let config = SimpleNameConfig::builder().min_length(10).build();
            let result = SimpleName::with_config("short".to_string(), config);
            assert!(matches!(result, Err(SimpleNameError::ValidationError(_))));
        }
    }
}
