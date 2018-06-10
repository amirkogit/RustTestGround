//! This module contains basic calculating functions.
//! 
//! # Examples
//! ```
//! let sum = math_lib::math::basic_calculator::add(2, 3);
//! let prod = math_lib::math::basic_calculator::product(8, 8);
//! let div = math_lib::math::basic_calculator::division(9, 3);
//! ```
//! 

/// Returns the sum of two i32 numbers
pub fn add(x: i32, y: i32) -> i32
{
    x + y
}

/// Returns the product of two i32 numbers
pub fn product(x: i32, y: i32) -> i32
{
    x * y
}

/// Returns the division of two i32 numbers
pub fn division(x: i32, y: i32) -> i32
{
    x / y
}