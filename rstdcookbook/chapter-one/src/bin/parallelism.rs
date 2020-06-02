// Parallelism through simple threads
// To run: cargo run --bin parallelism

use std::thread;

fn main(){
    println!(">>> Parallelism through simple threads.");

    // spawning a thread lets it executes a lambda
    let child = thread::spawn(|| println!("Hello from a new thread!"));

    println!("Hello from the main thread");

    child.join().expect("Failed to join the child thread");

    let sum = parallel_sum(&[1,2,3,4,5,6,7,8,9,10]);
    println!("The sum of numbers is: {}", sum);
}

// function to sum numbers in a slice in parallel
fn parallel_sum(range: &[i32]) -> i32 {
    const NUM_THREADS: usize = 4;

    // if input numbers < 4 then no need to use threads
    if range.len() < NUM_THREADS {
        sum_bucket(range)
    }
    else {
        // define "bucket" as amount of numbers we sum in a single thread
        let bucket_size = range.len() / NUM_THREADS;

        let mut count = 0;

        let mut threads = Vec::new(); // Vector to keep track of our threads

        while count + bucket_size < range.len() {
            let bucket = range[count..count + bucket_size].to_vec();
            let thread = thread::Builder::new()
                .name("Calculation".to_string())
                .spawn(move || sum_bucket(&bucket))
                .expect("Failed to create the thread");
            
            threads.push(thread);

            count += bucket_size;
        }

        // we are going to sum the rest in the main thread
        let mut sum = sum_bucket(&range[count..]);

        // add the results up
        for thread in threads {
            sum += thread.join().expect("Failed to join thread");
        }

        sum
    }
}

// this is the function that will be executed in thread
fn sum_bucket(range: &[i32]) -> i32 {
    //panic!(); // for debugging to make an app crash
    
    let mut sum = 0;
    for num in range {
        sum += *num;
    }

    sum
}