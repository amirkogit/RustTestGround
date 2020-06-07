// Using a slab
// To run: cargo run --bin slab

extern crate slab;
use slab::{Slab, VacantEntry};

fn main() {
    println!(">>> using slab...");

    // A slab is meant to be used as a limited buffer
    // as such, you should initialize it with a pre-defined capacity

    const CAPACITY: usize = 1024;

    let mut slab  = Slab::with_capacity(CAPACITY);

    // you cannot simply access a slab's entry by index of by searching it
    // instead, every insert gives you a key that can be used to access its entry
    let hello_key = slab.insert("hello");
    let world_key = slab.insert("world");

    println!("hello_key -> {}", slab[hello_key]);
    println!("world_key -> {}", slab[world_key]);

    // you can pass an "empty spot" around in order to be filled
    let data_key = {
        let entry = slab.vacant_entry();
        fill_some_data(entry)
    };
    println!("data_key -> {}", slab[data_key]);

    // ---------------------------------------
    // when iterating, you get a key-value pair

    for (key, value) in &slab {
        println!("{} --> {}", key, value);
    }

    // checking the slab capacity manually
    if slab.len() != slab.capacity() {
        slab.insert("the slab is not at capacity");
    }

    println!("slab: {:?}", slab);
}

fn fill_some_data(entry: VacantEntry<&str>) -> usize {
    let data = "Some data";

    // insert() consumes the entry, so we need to get the key before
    let key = entry.key();
    entry.insert(data);
    println!("inserted key = {}", key);
    key
}