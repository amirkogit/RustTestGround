// Here we create a trait called Shape. Other geometric objects like Square, Rectangle, Cube, Circle and Sphere will use this trait and provide its own specialized methods.

trait Shape
{
    fn create(name: &'static str, dimension: f64) -> Self;
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;
}

// Define geometric object Shape that will use trait->Shape
struct Square
{
    name: &'static str,
    length: f64,
}

// implementation of Shape trait for Square
impl Shape for Square
{
    fn create(name: &'static str, dimension: f64) -> Square {
        Square { name: name, length: dimension }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn area(&self) -> f64 {
        self.length * self.length
    }
}

// Square specific implementations
impl Square 
{
    fn length(&self) -> f64 {
        self.length
    }
}

// define geometric object Circle that will use trait->Shape
struct Circle
{
    name: &'static str,
    radius: f64
}

// implementation of trait->Shape for Circle
impl Shape for Circle
{
    fn create(name: &'static str, dimension: f64) -> Circle {
        Circle { name: name, radius: dimension }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn area(&self) -> f64 {
        let PI = 3.14;
        let area = PI * self.radius * self.radius;
        area
    }
}

// Circle specific implementation
impl Circle
{
    fn radius(&self) -> f64 {
        self.radius
    }
}

fn traits_demo()
{
    let square = Square { name: "square", length: 10.0 };
    let area = square.area();
    println!("Square => length = {} name = {} area = {}", square.length(), square.name, area);

    let circle = Circle { name: "mycircle", radius: 5.0 };
    println!("Circle => radius = {} name = {} area = {} ", circle.radius, circle.name, circle.area());

    // creating objects via factory methods
    let square2: Square = Shape::create("square2", 12.0);
    println!("Square => length = {} name = {} area = {}", square2.length(), square2.name, square2.area());

    let circle2: Circle = Shape::create("circle2", 3.2);
    println!("Circle => radius = {} name = {} area = {} ", circle2.radius, circle2.name, circle2.area());
}

fn main() 
{
    println!("Traits Demo");
    traits_demo();
}
