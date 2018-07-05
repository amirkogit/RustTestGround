// src/main.rs

extern crate libc;

extern 
{
	fn print();
    fn add(input: libc::c_int, input: libc::c_int) -> libc::c_int;
}

fn main() 
{
	unsafe { print(); }
	
    let result = unsafe { add(10,20) };
    println!("Sum: {}", result);
}
