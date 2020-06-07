// Reading from stdin
// To run: cargo run --bin stdin

use std::io;
use std::io::prelude::*;

fn main() {
    print_single_line("Enter firstname: ");
    let firstname = read_line_iter();

    print_single_line("Enter lastname: ");
    let lastname = read_line_buffer();

    print_single_line("Enter age: ");
    let age = read_number();

    println!("Hello, {} year old human named {} {}", age, firstname, lastname);
}

fn print_single_line(text: &str) {
    // we can print lines without adding a newline
    print!("{}", text);

    // however, we need to flush stdout afterwards inorder to guarantee that the data actually displays
    io::stdout().flush().expect("Failed to flush stdout");
}

fn read_line_iter() -> String {
    let stdin = io::stdin();

    // read onle line of input iterator-style
    let input = stdin.lock().lines().next();

    input
        .expect("No lines in buffer")
        .expect("Failed to read line")
        .trim()
        .to_string()
}

fn read_line_buffer() -> String {
    // read one line of input buffer-style
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn read_number() -> i32 {
    let stdin = io::stdin();

    loop {
        // iterate over all lines that will be inputed
        for line in stdin.lock().lines() {
            let input = line.expect("Failed to read line");

            // convert string into a number
            match input.trim().parse::<i32>() {
                Ok(num) => return num,
                Err(e) => println!("Failed to read number: {}", e),
            }
        }
    }
}