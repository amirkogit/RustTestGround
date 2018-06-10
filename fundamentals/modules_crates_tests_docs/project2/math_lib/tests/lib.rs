#[cfg(test)]
mod tests
{
    extern crate math_lib;

    #[test]
    fn add_correct()
    {
        let sum = math_lib::math::basic_calculator::add(2, 3);
        assert_eq!(5,sum);
    }

    #[test]
    fn add_incorrect()
    {
        let sum = math_lib::math::basic_calculator::add(12, 13);
        assert_ne!(5,sum);        
    }

    #[test]
    fn product_correct()
    {
        let prod = math_lib::math::basic_calculator::product(8, 8);
        assert_eq!(64, prod);
    }

    #[test]
    fn division_correct()
    {
        let div = math_lib::math::basic_calculator::division(9, 3);
        assert_eq!(3, div);
    }

    #[test]
    fn to_binary_correct()
    {
        let result = math_lib::math::scientific_calculator::to_binary(20);
        assert_eq!("binary string", result);
    }

    #[test]
    fn to_hexadecimal_correct()
    {
        let result = math_lib::math::scientific_calculator::to_hex(88);
        assert_eq!("hexadecimal string", result);
    }
}