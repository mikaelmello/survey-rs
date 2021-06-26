//! This module contains the type aliases for functions called as validators
//! of a given input.
//!
//! It also provides several built-in validators generated through macros.

use crate::answer::OptionAnswer;

/// Type alias for validators that receive a string slice as the input.
/// When creating containers of validators, you might need to type hint
/// them using this type.
pub type StringValidator = fn(answer: &str) -> Result<(), String>;

/// Type alias for validators that receive a collection of [OptionAnswer]'s as the input.
/// When creating containers of validators, you might need to type hint
/// them using this type.
pub type MultiOptionValidator = fn(answer: &[OptionAnswer]) -> Result<(), String>;

/// Built-in validator that checks whether the answer is not empty.
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! required {
    () => {
        $crate::required! {"A response is required."}
    };

    ($message:expr) => {
        |a| match a.is_empty() {
            true => Err(String::from($message)),
            false => Ok(()),
        }
    };
}

/// Built-in validator that checks whether the answer length is below or equal to the specified threshold.
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! max_length {
    ($length:expr) => {
        $crate::max_length! {$length, format!("The length of the response should be at most {}", $length)}
    };

    ($length:expr, $message:expr) => {
        {
            |a| match a.len() {
                _len if _len <= $length => Ok(()),
                _ => Err(String::from($message)),
            }

        }
    };
}

/// Built-in validator that checks whether the answer length is above or equal to the specified threshold.
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! min_length {
    ($length:expr) => {
        $crate::min_length! {$length, format!("The length of the response should be at least {}", $length)}
    };

    ($length:expr, $message:expr) => {
        {
            |a| match a.len() {
                _len if _len >= $length => Ok(()),
                _ => Err(String::from($message)),
            }
        }
    };
}

/// Built-in validator that checks whether the answer length is equal to the specified value.
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! length {
    ($length:expr) => {
        $crate::length! {$length, format!("The length of the response should be {}", $length)}
    };

    ($length:expr, $message:expr) => {{
        |a| match a.len() {
            _len if _len == $length => Ok(()),
            _ => Err(String::from($message)),
        }
    }};
}

/// Built-in validator that checks whether the answer matches the specified ReGeX pattern.
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! regex {
    ($regex:expr) => {
        $crate::regex! {$regex, format!("The response should match pattern {}", $regex)}
    };

    ($regex:expr,$message:expr) => {{
        |a| {
            use regex::Regex;
            let regex = Regex::new($regex).unwrap();
            if regex.is_match(a) {
                Ok(())
            } else {
                Err(String::from($message))
            }
        }
    }};
}

/// Built-in validator that checks whether the answer is able to be successfully parsed to a primitive type, such as f64.
#[macro_export]
#[cfg(feature = "builtin_validators")]
macro_rules! parse_primitive {
    ($type:ty) => {
        $crate::parse_primitive! {$type, format!("Failure when parsing response to type {}", std::any::type_name::<$type>())}
    };

    ($type:ty, $message:expr) => {{
        |a| match a.parse::<$type>() {
            Ok(_) => Ok(()),
            Err(err) => Err(String::from($message)),
        }
    }};
}
