// Using a vector
// To run: cargo run --bin vector

fn main() {
    println!(">>> Using a vector");

    // create a vector with some elements
    let fruits = vec!["apple", "tomato", "pear"];

    // debug print vectors
    println!("fruits: {:?}", fruits);

    // create an empty vector and fill it
    let mut fruits = Vec::new();
    fruits.push("water-melon");
    fruits.push("banana");
    fruits.push("kiwi");
    println!("fruits: {:?}", fruits);

    // remove the last element
    let last = fruits.pop();
    if let Some(last) = last {
        println!("Removed {} from {:?}", last, fruits);
    }
    println!("fruits: {:?}", fruits);

    // insert an element into the middle of the vector
    fruits.insert(1, "grape");
    println!("fruits after insertion: {:?}", fruits);

    // swap two elements
    fruits.swap(0,1);
    println!("fruits after swap: {:?}", fruits);

    // access the first and last element
    let first = fruits.first();
    if let Some(first) = first {
        println!("First fruit: {}", first);
    }
    let last = fruits.last();
    if let Some(last) = last {
        println!("Last fruit: {}", last);
    }

    // access arbitary elements
    let second = fruits.get(1);
    if let Some(second) = second {
        println!("Second fruit: {}", second);
    }

    // access arbitary elements without bounds checking
    let second = fruits[1];
    println!("Second fruit: {}", second);

    // -------------------------------------------------

    // initialize a vector with a value with zeros
    let zeros = vec![0;5];
    println!("zeros: {:?}", zeros);

    // remove some item and shift all that come after into place
    let mut nums = vec![1,2,3,4];
    println!("nums: {:?}", nums);
    let second_num = nums.remove(1);
    println!("Removed {} from {:?}", second_num, nums);

    // filter the vector in place
    let mut names = vec!["aron", "john", "alex", "cathy", "bob", "susan"];
    println!("names: {:?}", names);

    // only keep names starting with 'a'
    names.retain(|name| name.starts_with('a')); // case sensitive
    println!("names: {:?}", names);

    // check if the vector contains an element
    println!("Does 'names' contain \"alex\" ? {}", names.contains(&"alex"));

    // remove consecutive duplicates
    let mut nums = vec![1,2,2,3,4,4,4,4,5,6];
    nums.dedup();
    println!("Duduped, pre-sorted nums: {:?}", nums);

    // sort a vector
    let mut nums = vec![3,5,1,2,8,12,4];
    println!("unsorted nums: {:?}", nums);
    nums.sort();
    println!("Sorted nums: {:?}", nums);

    // reverse a vector
    nums.reverse();
    println!("reversed nums: {:?}", nums);

    // create a consuming iterator over a range
    let mut alphabet = vec!['a', 'b', 'c', 'd', 'e'];
    print!("First two letters of alphabet are: "); // print! will print without a new line
    for letter in alphabet.drain(..2) {
        print!("{}", letter);
    }
    println!();

    // the drained elements are no longer in the vector
    println!("alphabet after being drained: {:?}", alphabet);

    // check if the vector is empty
    let mut fridge = vec!["Beer", "Leftovers", "Oranges"];
    println!("Is fridge empty? {}", fridge.is_empty());

    // remove all elements
    fridge.clear();
    println!("Is fridge empty? {}", fridge.is_empty());
    println!("fridge contents: {:?}", fridge);

    // -------------------------------------------------

    // split a vector into two
    let mut colors = vec!["red", "blue", "green", "yellow"];
    println!("colors before split: {:?}", colors);

    let half = colors.len() / 2;
    let mut second_half = colors.split_off(half);

    println!("colors after split: {:?}", colors);
    println!("second half: {:?}", second_half);

    // put vectors together
    colors.append(&mut second_half);
    println!("colors after append: {:?}", colors);
    println!("second half: {:?}", second_half);

    // -------------------------------------------------

    // splice method from javascript?
    let mut stuff = vec!["1","2","3","4","5","6"];
    println!("Original stuff: {:?}", stuff);

    let stuff_to_insert = vec!["a", "b", "c"];

    let removed_stuff: Vec<_> = stuff.splice(1..4, stuff_to_insert).collect();

    println!("spliced stuff: {:?}", stuff);
    println!("removed stuff: {:?}", removed_stuff);

    // -------------------------------------------------

    // initialize a vector with a certain capacity
    let mut large_vec: Vec<i32> = Vec::with_capacity(1_000_000);
    println!("large_vec after creation:");
    println!("len: \t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // shrink the vector
    large_vec.shrink_to_fit();
    println!("large_vec after shrinking:");
    println!("len: \t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // remove some item, replacing it with the last
    let mut nums = vec![1,2,3,4];
    println!("nums: {:?}", nums);
    let second_num = nums.swap_remove(1); // this changes the order, but works in O(1)
    println!("removed {} from {:?}", second_num, nums);
}