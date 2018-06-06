use std::mem;

// private struct not accessible to outside module
struct Point2D
{
    x: f64,
    y: f64
}

// private method not accessbile to outside module
fn get_origin() -> Point2D
{
    Point2D { x: 0.0, y: 0.0}
}

// define a public method accessible to other modules
pub fn stack_heap_demo()
{
    println!("Stack and Heap demo");

    let p1 = get_origin(); // allocates memory on stack
    let p2 = Box::new(get_origin()); // allocates memory on heap

    println!("p1 takes {} bytes", mem::size_of_val(&p1)); 
    println!("p2 takes {} bytes", mem::size_of_val(&p2)); // returns size of pointer

    let p3 = *p2;
    println!("p3 takes {} bytes", mem::size_of_val(&p3));

    println!("p3.x = {} p3.y = {}", p3.x, p3.y);

    let d1 = Point2D {x:5.1, y:8.2};
    println!("d1.x = {} d1.y = {}",d1.x, d1.y);
}
