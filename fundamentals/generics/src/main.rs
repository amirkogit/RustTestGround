// suppose we have a coordinate that represents point in 2D. 
// A point could be int or double.
// A line can be composed of two points - start and end

struct Point2D<T>
{
    x: T,
    y: T
}

struct Line2D<T>
{
    start: Point2D<T>,
    end: Point2D<T>
}

fn lines_with_int()
{
    println!("line with i32 values:");
    let a = Point2D { x: 20, y: 10 };
    let b = Point2D { x: 40, y: 50 };
    println!("a = ({},{})", a.x, a.y);
    println!("b = ({},{})", b.x, b.y);
    let line = Line2D { start: a, end: b};
    println!("line = (({},{}),({},{}))", line.start.x, line.start.y, line.end.x, line.end.y);
}

fn lines_with_double()
{
    println!("line with f64 values:");
    let a = Point2D { x: 20.56, y: 10.554 };
    let b = Point2D { x: 40.978, y: 50.78 };
    println!("a = ({},{})", a.x, a.y);
    println!("b = ({},{})", b.x, b.y);
    let line = Line2D { start: a, end: b};
    println!("line = (({},{}),({},{}))", line.start.x, line.start.y, line.end.x, line.end.y);
}

// generic function
fn foo<T>(arg: T)
{

}

// suppose we have following structures
struct Value {
    val: f64
}

struct GenericValue<T> {
    gen_val: T
}

// impl of Value
impl Value {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenericValue for a generic type
impl<T> GenericValue<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn generic_values()
{
    println!("generic value demo:");
    let x = Value { val: 6.56 };
    let y = GenericValue { gen_val: 3i32 };
    println!("x(Value) = {} y(GenericValue) = {}", x.value(), y.value());
    let z = GenericValue { gen_val: 3.14 };
    println!("z(GenericValue) = {}", z.value());
    let some_string = GenericValue { gen_val: "random string" };
    println!("some_string(GenericValue) = {}", some_string.value());
}

fn main() 
{
    println!("Generics Demo");
    lines_with_int();
    lines_with_double();
    generic_values();
}
