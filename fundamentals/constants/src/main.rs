use std::mem;

// define some constants
const MAGIC_NUMBER:u8 = 42;

// global variable
static mut PASSCODE:i32 = 6035; // thread unsafe

fn main() {
    println!("Constants demo");
    println!("MAGIC_NUMBER = {}", MAGIC_NUMBER);

    // cannot change the value of constant
   // MAGIC_NUMBER = 34;

    // cannot compile the following code. It has to be wrapped in unsafe block
    //println!("PASSCODE = {}", PASSCODE);

    unsafe {
        println!("PASSCODE = {}", PASSCODE);

        PASSCODE = 8789;
        println!("New PASSCODE = {}", PASSCODE);
    }
}
