// Using a HashMap
// To run: cargo run --bin hashmap

use std::collections::HashMap;

fn main() {
    // the HashMap can map any hashable type to an other
    // the first type is called "key"
    // second one is called "value"

    let mut tv_ratings = HashMap::new();
    tv_ratings.insert("The IT crowd", 8);
    tv_ratings.insert("13 reasons why", 7);
    tv_ratings.insert("House of Cards", 9);
    tv_ratings.insert("Stranger Things", 8);
    tv_ratings.insert("Breaking Bad", 10);

    println!("tv_ratings: {:?}", tv_ratings);

    // does a key exists?
    let contains_tv_show = tv_ratings.contains_key("House of Cards");
    println!("contains_tv_shows (House of Cards): {}", contains_tv_show);

    let contains_tv_show = tv_ratings.contains_key("House");
    println!("contains_tv_shows(Hoouse) : {}", contains_tv_show);

    println!("tv_ratings: {:?}", tv_ratings);

    // access a value
    if let Some(rating) = tv_ratings.get("Breaking Bad") {
        println!("I rate Breaking Bad {}", rating);
    }

    // if we insert a value twice, we overwrite it
    let old_rating = tv_ratings.insert("13 reasons why", 9);
    println!("tv_ratings: {:?}", tv_ratings);

    // remove a key and its value
    let removed_value = tv_ratings.remove("The IT crowd");
    if let Some(removed_value) = removed_value {
        println!("Removed series had a rating of {}", removed_value);
    }

    println!("tv_ratings: {:?}", tv_ratings);

    // ---------------------------------------------------

    // iterating all keys and values
    println!("All ratings");
    for (key, value) in &tv_ratings {
        println!("{}\t: {}", key, value);
    }

    // we can iterate mutably
    println!("All ratings with 100 as maximum");
    for (key, value) in &mut tv_ratings {
        *value *= 10;
        println!("{}\t: {}", key, value);
    }

    // iterating without referenceing the HashMap moves its content
    for _ in tv_ratings {}
    // tv_ratings is not usable anymore
    //println!("tv_ratings: {:?}",tv_ratings);

    // ---------------------------------------------------

    // if you don't need to access both keys and values at the same time, you can iterate over either individually

    let mut age = HashMap::with_capacity(10);
    age.insert("Dory", 8);
    age.insert("Nemo", 5);
    age.insert("Merlin", 10);
    age.insert("Bruce", 9);

    // iterate over all keys
    println!("All names:");
    for name in age.keys() {
        println!("{}", name);
    }

    // iterate over all values
    println!("All values:");
    for age in age.values() {
        println!("{}", age);
    }

    // iterate over all values and mutate them
    println!("All ages in 10 years:");
    for age in age.values_mut() {
        *age += 10;
        println!("{}", age);
    }

    // --------------------------------------------------
    // using entry API to assign default values to keys if they are not yet in the HashMap
    println!("age: {:?}", age);
{
    let age_of_coral = age.entry("coral").or_insert(11);
    println!("age_of_coral: {}", age_of_coral);

}

    println!("age: {:?}", age);

    let age_of_coral = age.entry("coral").or_insert(15);
    println!("age_of_coral: {}", age_of_coral);

    println!("age: {:?}", age);
}