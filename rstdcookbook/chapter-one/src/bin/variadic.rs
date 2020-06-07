// Accepting variable number of arguments
// To run: cargo run --bin variadic

macro_rules! multiply {
    // edge case
    ($last:expr) => {
        $last
    };

    ($head:expr, $($tail:expr), +) => {
        // recursive call
        $head * multiply!($($tail), +)
    };
}

fn main() {
    let val = multiply!(2,4,8);
    println!("2*4*8 = {}", val);

    println!("4*5 = {}", multiply!(4,5));
    println!("3 = {}", multiply!(3));
}