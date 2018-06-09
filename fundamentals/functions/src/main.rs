// function that takes a tuple as an argument
fn print_value(pair:(i32,i32))
{
    println!("pair = {:?}", pair);
}

// function that takes a mutable reference
fn add(x: &mut i32)
{
    *x += 1
}

// function that takes two arguments and returns a value
fn product(x: f32, y: f32) -> f32
{
    x * y // no semicolon
}

struct point2D {
    x: f64,
    y: f64
}

impl point2D
{
    fn print_val(&self)
    {
        println!("({},{})", self.x, self.y);
    }
}

// find distance between two points
fn find_distance(p1:point2D, p2:point2D) -> f64
{
    let dist:f64 = (p2.x - p1.x) * (p2.x - p1.x) + 
                (p2.y - p1.y) * (p2.y - p1.y);
    dist
}

fn main() 
{
    println!("Functions Demo");
    let pair = (32,64);
    print_value(pair);

    let mut a = 100;
    println!("a = {}", a);
    add(&mut a);
    println!("a after add() = {}", a);

    println!("product of {} * {} = {}", 5.6, 8.4, product(5.6, 8.4));

    let p1: point2D = point2D { x: 8.6, y: 9.3 };
    let p2: point2D = point2D { x: 7.1, y: 2.14 };
    p1.print_val();
    p2.print_val();    
    let dist = find_distance(p1, p2);
    println!("distance = {}", dist);
}
