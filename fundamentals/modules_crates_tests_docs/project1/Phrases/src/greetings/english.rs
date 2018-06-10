//! This module contains English phrases.
//!
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}!",
//!   Phrases::greetings::english::hello(),
//!   username);
//! ```

/// Applies to code that follows it.
/// In this case, it's our `hello()` function.
pub fn hello() -> String { return "hello".to_string(); /* this is easy */ }
pub fn goodbye() -> String { return "goodbye".to_string(); }