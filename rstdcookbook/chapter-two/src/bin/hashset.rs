// Using HashSet
// To run: cargo run --bin hashset

use std::collections::HashSet;

fn main() {
    // most of the interfaces of HashSet is the same as HashMap, just
    // without the methods that handle values

    let mut books = HashSet::new();
    books.insert("Harry Potter");
    books.insert("A Game of Thrones");
    books.insert("The Name of the Wind");

    println!("books: {:?}", books);

    // A HashSet will ignore duplicate entries
    // but it will return if an entry is new or not found
    let is_new = books.insert("Pragmatic Programmer");
    if is_new {
        println!("Added an new book");
        println!("books: {:?}", books);
    }

    let is_new = books.insert("Harry Potter");
    if !is_new {
        println!("The book already exists!");
    }

    // check if it contains a key
    if !books.contains("The Doors of Stone") {
        println!("Book not present");
    }

    if books.contains("Harry Potter") {
        println!("Harry Potter is available");
    }

    // remove an entry
    let was_removed = books.remove("Some other books");
    println!("was_removed: {}", was_removed);
    
    let was_removed = books.remove("Harry Potter");
    println!("was_removed: {}", was_removed);
    println!("books: {:?}", books);

    // --------------------------------------------------
    // compare different HashSets

    let one_to_five: HashSet<_> = (1..6).collect();
    let five_to_ten: HashSet<_> = (5..11).collect();
    let one_to_ten: HashSet<_> = (1..11).collect();
    let three_to_eight: HashSet<_> = (3..9).collect();

    // check if two HashSets have no elements in common
    let is_disjoint = one_to_five.is_disjoint(&five_to_ten);
    println!(
        "is {:?} disjoint fron {:?}? : {}",
        one_to_five,
        five_to_ten,
        is_disjoint
    );

    let is_disjoint = one_to_five.is_disjoint(&three_to_eight);
    println!(
        "is {:?} disjoint from {:?}? : {}",
        one_to_five,
        three_to_eight,
        is_disjoint
    );

    // check if a HashSet is fully contained in another
    let is_subset = one_to_five.is_subset(&one_to_ten);
    println!(
        "is {:?} a subset of {:?}? : {}",
        one_to_five,
        one_to_ten,
        is_subset
    );

    // check if a HashSet fully contains another
    let is_superset = three_to_eight.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?} ? : {}",
        three_to_eight,
        five_to_ten,
        is_superset
    );

    let is_superset = one_to_ten.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?} ? : {}",
        one_to_ten,
        five_to_ten,
        is_superset
    );

    // ------------------------------------------------------
    // join two HashSets in various ways

    // get the values that are in the first HashSet but not in the second
    let difference = one_to_five.difference(&three_to_eight);
    println!(
        "Difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        difference
    );

    // get the values that are in either HashSets, but not in both
    let symmetric_difference = one_to_five.symmetric_difference(&three_to_eight);
    println!(
        "Symmetric difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        symmetric_difference
    );

    // get the values that are in both HashSets
    let intersection = one_to_five.intersection(&three_to_eight);
    println!(
        "Intersection between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        intersection
    );

    // get all values in both HashSets
    let union = one_to_five.union(&three_to_eight);
    println!(
        "Union between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        union
    );
}