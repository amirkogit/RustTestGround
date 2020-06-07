// Accessing collections as iterators
// To run: cargo run --bin iterators

fn main() {
    // a string itself is not iterable, but its characters are
    let mut alphabet = "ABCDEFGHIJKLMNOPWRSTUVWXYZ".chars();
    // for letter in alphabet {
    //     print!("{}.", letter);
    // }
    // println!();

    // ranges are also (limited) iterators
    let nums = 0..10;
    let all_nums = 0..; // infinite iterators

    for n in nums {
        print!("{}", n);
    }
    println!();

    // get the index of the current item
    for (index, letter) in "abcd".chars().enumerate() {
        println!("#{}. letter in the alphabet: {}", index + 1, letter);
    }

    // ------------------------------------------------------

    // going through an iterator, step by step
    let names = vec!["Joe", "Miranda", "Alice"];
    println!("names: {:?}", names);

    let mut iter = names.iter();
    println!("{:?}", iter);

    if let Some(name) = iter.next() {
        println!("First name: {}", name);
    }

    if let Some(name) = iter.next() {
        println!("Second name: {}", name);
    }

    if let Some(name) = iter.next() {
        println!("Third name: {}", name);
    }

    if iter.next().is_none() {
        println!("No names left");
    }

    // arbitary access to an item in the iterator
    let letter = alphabet.nth(3);
    if let Some(letter) = letter {
        println!("fourth letter in alphabet: {}", letter);
    }

    // this works by consuming all items up to a point. So for the following two codes, it will result in two different return values
    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        println!("first item: {}", current_first); // this will not print 'A'
    }

    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        println!("First item: {}", current_first);
    }

    // accessing the last item, this will consume the entire iterator
    let last_letter = alphabet.last();
    if let Some(last_letter) = last_letter {
        println!("last letter: {}", last_letter);
    }

    // -----------------------------------------------------

    // collect iterators into collections
    // this requires an annotation of which collection we want
    // the following two are equivalent
    let nums: Vec<_> = (1..10).collect();
    println!("nums: {:?}", nums);
    let nums = (1..10).collect::<Vec<_>>();
    println!("nums: {:?}", nums);

    // taking only the first n items. this is often used to make an infinite iterator finite
    let nums: Vec<_> = all_nums.take(5).collect(); // let all_nums = 0..; // infinite iterators
    println!("first five numbers: {:?}", nums);

    // skip the first few items
    let nums: Vec<_> = (0..11).skip(4).collect();
    println!("skipped: {:?}", nums);

    // take and skip accept predicates in the form of
    // take_while and skip_while
    let nums: Vec<_> = (0..).take_while(|x| x*x < 50).collect();
    println!("All positive numbers that are less than 50 when squared: {:?}", nums);

    // this is useful to filter an already sorted vector
    let names = ["Alfred", "Andy", "Jose", "Luke"];
    let names: Vec<_> = names.iter().skip_while(|x| x.starts_with('A')).collect();
    println!("Names that don't start with 'A': {:?}", names);

    // filtering iterators
    let countries = [
        "USA",
        "Germany",
        "France",
        "Italy",
        "Nepal",
        "Japan",
        "India",
        "Pakistan",
        "Burma",
    ];

    let countries_with_i: Vec<_> = countries
        .iter()
        .filter(|country| country.contains('i') || country.contains('I'))
        .collect();

    println!("All countries containing letter 'i': {:?}", countries_with_i);

    // ------------------------------------------------------------

    // check if an iterator contains an element
    // find the first element that satisfies a condition
    if let Some(country) = countries.iter().find(|country| country.starts_with('I')) {
        println!("First country starting with letter 'I': {}", country);
    }

    // don't get the searched item but rather its index
    if let Some(pos) = countries.iter().position(|country| country.starts_with('I')) {
        println!("First contry starting with letter 'I' index only: {}", pos);
    }

    // check if at least one item satisfies a condition
    let are_any = countries.iter().any(|country| country.len() == 5);
    println!("Is there at least one contry with exactly 5 letters? {}", are_any);

    // check if all items satisfy a condition
    let are_all = countries.iter().all(|country| country.len() == 5);
    println!("Do all countries have exactly 5 letters? {}", are_all);

    // -------------------------------------------------------------
    
    // useful operations for numeric items
    let sum: i32 = (1..11).sum();
    let product: i32 = (1..11).product();
    println!("sum: {} product: {}", sum, product);

    let max = (1..11).max();
    let min = (1..11).min();
    if let Some(max) = max {
        println!("max no: {}", max);
    }

    if let Some(min) = min {
        println!("min no: {}", min);
    }

    // -----------------------------------------------------------

    // combine iterators
    // combine an iterator with itself, making it infinite when it 
    // reaches its end, it starts again
    let some_numbers: Vec<_> = (1..4).cycle().take(10).collect();
    println!("some_numbers: {:?}", some_numbers);

    // combine two iterators by putting them after another
    let some_numbers: Vec<_> = (1..4).chain(10..14).collect();
    println!("some_numbers: {:?}", some_numbers);

    // zip two iterators together by grouping their first items together
    // their second items together etc.
    let swiss_post_codes = [8957, 5000, 5034, 8989];
    let swiss_towns = ["Spreitenbach", "Aarau", "Suhr", "ddd"];
    let zipped: Vec<_> = swiss_post_codes.iter().zip(swiss_towns.iter()).collect();
    println!("zipped: {:?}", zipped);

    // because zip is lazy, we can use infinte ranges
    let zipped: Vec<_> = (b'A'..)
        .zip(1..)
        .take(10)
        .map(|(ch,num)| (ch as char, num))
        .collect();
    println!("zipped: {:?}", zipped);

    // -------------------------------------------------------
    // apply function to all items

    // change items type
    let numbers_as_strings: Vec<_> = (1..11).map(|x| x.to_string()).collect();
    println!("numbers_as_strings: {:?}", numbers_as_strings);

    // access all items
    println!("First ten squares:");
    (1..11).for_each(|x| print!("{} ", x*x));
    println!();

    // filter and map items at the same time
    let squares: Vec<_> = (1..50).filter_map(|x| if x % 3 == 0 { Some(x * x) } else { None }).collect();
    println!("squares of all numbers < 50 that are divisible by 3: {:?}", squares);

    // -----------------------------------------------------------

    // retrieve the entire alphabet in lower and uppercase
    let alphabet: Vec<_> = (b'A' .. b'z' + 1) // add 1 to include 'z'
        .map(|c| c as char) // convert all to characters
        .filter(|c| c.is_alphabetic()) // filter only alphabetic chars
        .collect(); // Collect as Vec<char>
    println!("alphabet: {:?}", alphabet);
}