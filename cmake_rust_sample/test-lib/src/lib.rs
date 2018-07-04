
#[no_mangle]
pub extern "C" fn greetings() 
{
    println!("Hello from Rust library file!");
}

#[no_mangle]
pub extern "C" fn add(x:i32, y:i32) -> i32
{
  x + y
}
