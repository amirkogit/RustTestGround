#![allow(dead_code)]
#![allow(unused_variables)]

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    CmykColor { cyan:u8, magenta:u8, yellow:u8, black:u8}
}

fn enums_demo()
{
    let color_val1 = Color::Red;
    println!("Input color = Color::Red");

    match color_val1 {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        _ => println!("Default color")
    }

    let color_val2 = Color::CmykColor { cyan: 0, magenta: 128, yellow: 0, black: 255};

    match color_val2 {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RgbColor(0,0,0) | Color::CmykColor { cyan: _, magenta: _, yellow: _, black: 255} => println!("Black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r,g,b),
        _ => ()
    }
}

fn main() {
    println!("Enumerations demo");

    enums_demo();
}
