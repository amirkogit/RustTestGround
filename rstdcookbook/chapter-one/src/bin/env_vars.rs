// Interacting with environment variables
// To run: cargo run --bin env_vars

use std::env;

fn main() {
    println!(">>> Interacting with environment variables.");

    println!("Listing all env vars:");
    for (key, val) in env::vars() {
        println!("{} : {}", key, val);
    }

    // setting an env var for current process
    let key = "PORT";
    println!("Setting env var {}", key);
    env::set_var(key, "8080");

    print_env_var(key);

    // removing an env var for current process
    println!("Removing env var {}", key);
    env::remove_var(key);

    print_env_var(key);
}

fn print_env_var(key: &str) {
    match env::var(key) {
        Ok(val) => println!("{} : {}", key, val),
        Err(e) => println!("Couldn't print env var {} :{}", key, e),
    }
}