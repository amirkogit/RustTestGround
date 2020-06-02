// Querying with regexes
// To run: cargo run --bin regex

extern crate regex;

fn main() {
    println!(">>> Querying with regexes");

    use regex::Regex;

    // beginning a string with 'r' makes it a raw string
    let date_regex = 
        Regex::new(r"^\d{2}.\d{2}.\d{4}$").expect("Failed to create regex");
    
    let date = "06.20.2018";

    let is_date = date_regex.is_match(date);
    println!("Is '{}' a date? {}", date, is_date);

    let date2 = "077.556.898";
    let is_date2 = date_regex.is_match(date2);
    println!("Is '{}' a date? {}", date2, is_date2);
}