// Using format! macro
// To run: cargo run --bin format

fn main() {
    println!(">>> Using format! macro");

    let color = "red";
    let favorite = format!("Favorite color is {}", color);
    println!("{}", favorite);   

    // adding multiple parameters
    let hello = "hello";
    let world = "world!!!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world);

    // format! can concatenate any data types that implement the 'Display' trait
    let favorite_no = format!("Favorite number is : {}", 9);
    println!("{}", favorite_no);

    // including some parameters multiple times using positional parameters
    let duck_duck_goose = format!("{0},{0},{0},{1}", "duck", "goose");
    println!("{}", duck_duck_goose);
    let new_duck = format!("{1},{0},{1}", "duck", "goose");
    println!("{}", new_duck);

    // named parameters --just like in python!
    let introduction = format!(
        "My name is {lastname}, {firstname}",
        lastname = "Shrestha",
        firstname = "Amir"
    );
    println!("{}", introduction);

    let weather = format!(
        "Today's temperature is : {value} {unit}",
        value = 104.25,
        unit = "Farenheit"
    );
    println!("{}", weather);
}

