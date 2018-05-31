use std::mem;

fn var_and_types() {
	let a:u8 = 123; // unsigned, immutable
	println!("a = {}", a);

	// this won't compile as we are trying to re-assign value to an immutable one
	//a = 555;

	let mut b:u16 = 345; // unsigned, mutable
	println!("b before = {}", b);
	b = 563;
	println!("b after = {}",b);

	// out of range value for b
	b = 9999993333;
	println!("b out of range = {}", b); // the value that goes into b is truncted.

	let mut c = 123456;
	println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

	c = 12;
	println!("after modification c = {}, size = {} bytes", c, mem::size_of_val(&c));

	let z:isize = 123;
	let size_of_z = mem::size_of_val(&z);
	println!("z = {} takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

	let d:char = 'a';
	println!("d = {} is char, size = {} bytes", d, mem::size_of_val(&d));

	let e:f32 = 3.14;
	println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

	let f:f64 = 2.44;
	println!("e = {}, size = {} bytes", f, mem::size_of_val(&f));

	let g = false;
	println!("g = {}, size = {} bytes",g,mem::size_of_val(&g));

	// other stuffs taken from rustlang documentation

	// variables can be type annoated 
	let logical: bool = true;
	println!("logical = {}, size = {} bytes", logical, mem::size_of_val(&logical));

	let a_float: f64 = 1.9; // regular annotation
	let an_integer = 5i32; // suffix annotation, value = 5 type = i32
	println!("an_integer = {}, size = {} bytes", an_integer, mem::size_of_val(&an_integer));

	// default for float is f64 and default for integer is i32
	let default_float = 5.44;
	let default_integer = 12;
}

fn main() {
	var_and_types();
}
