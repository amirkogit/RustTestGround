fn operators_demo()
{
    println!("Operators demo");

    // arithmetic operators
    let mut a = 6+8*9;
    println!("a = {}",a);

    a = a + 1;
    //a++; // not possible
    //a--; // not possible

    a += 2;
    println!("a = {}",a);

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_square = i32::pow(a,2);
    println!("{} square = {}", a, a_square);

    let b = 3;
    let b_cubed = i32::pow(b,3);
    println!("{} cubed = {}", b, b_cubed);

    let c = 2.5;
    let c_cubed = f64::powi(c,3);
    let c_to_pi = f64::powf(c, std::f64::consts::PI);
    println!("{} cubed = {}, {} ^ pi = {}", c, c_cubed, c, c_to_pi);

    // bitwise operators
    let d = 1 | 2;
    println!("1 | 2 = {} , inverted d = {}", d, !d);

    //let e = 2.3 | 5.6; // will not compile

    // logical operator
    let pi_less_4 = std::f64::consts::PI < 5.0;
    println!("{}", pi_less_4);

    println!("5 < 6 = {}", 5 < 6);
    println!("5 < 2 = {}", 5 < 2);
}

fn main() 
{
    operators_demo();    
}
