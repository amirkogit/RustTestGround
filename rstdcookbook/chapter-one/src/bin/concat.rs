// Different ways to concatenate strings
// To run: cargo run --bin concat 

fn main() {
    by_moving();
    by_cloning();
    by_mutating();
}

fn by_moving() {
    println!(">>>> by_moving()");

    let hello = "hello ".to_string();
    let world = "world!";

    // moving hello into a new variable
    let hello_world = hello + world;

    println!("{}", hello_world);

    // the following is an error because value has been borrowed after move
    //println!("{}", hello);
}

fn by_cloning() {
    println!(">>>> by_cloning()");

    let hello = "hello ".to_string();
    let world = "world!";

    // creating a copy of hello nd moving it into a new variable
    let hello_world = hello.clone() + world;

    println!("{}", hello_world);

    // hello can still be used
    println!("{}", hello);
}

fn by_mutating() {
    println!(">>>> by_mutating()");

    let mut hello = "hello ".to_string();
    let world = "world!";

    // hello gets modified in place
    hello.push_str(world);

    // hello is both usable and modifiable
    println!("{}", hello);
}