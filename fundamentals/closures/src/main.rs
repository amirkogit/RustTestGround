// simple function
fn greetings()
{
    println!("hello world!");
}

fn simple_closure()
{
    // function assigned to some variable
    let greet = greetings;
    greet(); // can be invoked as a method
}

fn closure_inline()
{
    // define a closure for adding two to a given argument
    let add_two = |x:i32| -> i32 { x + 2 };
    let n = 10;
    println!("{} + 2 = {}", n, add_two(n));
}

fn closure_scoped()
{
    let two = 2;
    let add_two = |x| {
        let mut z = x;
        z += two;
        z
    };

    println!("{} + 2 = {}", 3, add_two(3));
    println!("two = {}", two);

    // following cannot be done!
    // let borrow_two = &mut two;
    // println!("borrow_two = {}", borrow_two);
}

fn main() 
{
    println!("Closures Demo");
    simple_closure();
    closure_inline();
    closure_scoped();
}
