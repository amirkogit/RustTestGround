// Accessing the command line
// To run: cargo run --bin cli_params <option1> <option2> ...

use std::env;

fn main() {
    println!(">>> Accessing the command line");
    
    //env::args returns an iterator over the parameters
    println!("Got the following parameters: ");
    for arg in env::args() {
        println!("- {}", arg);
    }

    // we can access specific parameters using the iterator API
    let mut args = env::args();
        
    if let Some(arg) = args.nth(0) {
        println!("The path to this program is : {}", arg);
    }

    if let Some(arg) = args.nth(1) {
        println!("The first parameter is : {}", arg);
    }

    if let Some(arg) = args.nth(2) {
        println!("The second parameter is : {}", arg);
    }

    // or as a vector
    let args: Vec<_> = env::args().collect();
    println!("The path to the program is : {}", args[0]);

    if args.len() > 1 {
        println!("First parameter is: {}", args[1]);
    }

    if args.len() > 2 {
        println!("Second parameter is: {}", args[2]);
    }

    if args.len() > 3 {
        println!("Third parameter is: {}", args[3]);
    }
}