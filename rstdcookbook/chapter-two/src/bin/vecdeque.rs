// Using a VecDeque
// To run: cargo run --bin vecdeque

use std::collections::VecDeque;

fn main() {
    // A VecDeque is best thought of as a FIFO queue
    let mut orders = VecDeque::new();
    println!("A guest ordered oyesters");
    orders.push_back("oysters");

    println!("A guest orderd fish and chips");
    orders.push_back("fish and chips");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    println!("A guest ordered mozarella sticks");
    orders.push_back("mozarella sticks");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    println!("A guest ordered onion rings!");
    orders.push_back("onion rings");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }
    else{
        println!("All orders delivered!");
    }

    // we can freely switch the pushing from front to back and vice versa
    let mut sentence = VecDeque::new();
    sentence.push_back("a");
    sentence.push_front("had");
    sentence.push_back("little");
    sentence.push_front("Marry");
    sentence.push_back("Lamb");
    println!("sentence: {:?}", sentence);

    // same applies to popping data
    sentence.pop_front();
    sentence.push_front("Jimmy");
    sentence.pop_back();
    sentence.push_back("Cat");
    println!("sentence: {:?}", sentence);

    // rest of VecDeque's methods are pretty much same as Vec
    // however, VecDeque has additional options when swap removing
    let mut some_queue = VecDeque::with_capacity(5);
    some_queue.push_back("A");
    some_queue.push_back("B");
    some_queue.push_back("C");
    some_queue.push_back("D");
    some_queue.push_back("E");
    println!("some_queue: {:?}", some_queue);

    some_queue.swap_remove_back(2);
    println!("some_queue after swap_remove_back: {:?}", some_queue);

    // swaps the removed element with the first one instead of last one
    some_queue.swap_remove_front(2);
    println!("some_queue after swap_remove_front: {:?}", some_queue);
}