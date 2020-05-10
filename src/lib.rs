//! This crate is a basic one only for testing TravisCI with rust
//!
//! It only implements two basic functions for greetings
//!
//! # Examples
//! ```
//! use rust_travis_example1;
//!
//! assert_eq!(rust_travis_example1::basic_greeting(), "Hello World!");
//! assert_eq!(rust_travis_example1::greeting_name("German"), "Hello German!");
//! ```

mod greetings_test;

pub fn basic_greeting() -> &'static str {
    "Hello World!"
}

pub fn greeting_name(name: &str) -> String {
    format!("Hello {}!", name)
}
