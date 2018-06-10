extern crate math_lib;
use math_lib::math::basic_calculator;
use math_lib::math::scientific_calculator;

fn main() 
{
    println!("math_lib demo...");

    let sum = math_lib::math::basic_calculator::add(4, 5);
    println!("sum = {}", sum);

    let prod = basic_calculator::product(8, 9);
    println!("product = {}", prod);

    let div = basic_calculator::division(25, 5);
    println!("division = {}", div);    

    println!("binary of {} = {}", 20, scientific_calculator::to_binary(20));
    println!("hex of {} = {}", 16, scientific_calculator::to_hex(16));
}
