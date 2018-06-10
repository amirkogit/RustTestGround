//! Math module consists of general math utilities and calculating functions.
//! 

/// common math package under math_lib
pub mod math
{
    pub mod basic_calculator;
    pub mod scientific_calculator;

    pub mod utils
    {
        //! Utils consists of various mathematical constants and formulae. 
        
        /// Returns the constant value for PI
        pub fn getPI() -> f64 {
            3.14
        }
    }
}