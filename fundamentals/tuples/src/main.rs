fn sum_and_product(x:f64, y:f64) -> (f64, f64)
{
    (x+y, x*y)
}

fn tuples_demo()
{
    let x = 4.5;
    let y = 5.6;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2} and {0} * {1} = {3}", x, y, sp.0, sp.1);

    // nice way to destructuring tuple values
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // we can have tuples of tuples
    let sp2 = sum_and_product(1.2, 33.2);
    let combined = (sp, sp2);
    println!("combined = {:?}", combined);
    println!("last element = {}",(combined.1).1);

    // destructuring combined
    let ((c,d),(e,f)) = combined;
    println!("c = {} d = {} e = {} f = {}", c, d, e, f);

    // tuple of different element types
    let person = ("name", 30, 5000.8);
    println!("person = {:?}", person);

    // representing a single tuple
    let single_tuple = (560,); // add comma in the end, otherwise it will be considered a fundamental data type
    println!("{:?}", single_tuple);
}

fn points_with_tuple()
{
    println!("points represented with tuple");
    let point1 = (2.0, 3.2);
    println!("point1 = {:?}", point1);
    let point2 = (3.2,9.5);
    println!("point2 = {:?}", point2);
    let (x,y) = point1;
    println!("x = {} y = {}", x, y);
}

fn main() 
{
    println!("Tuples demo");
    tuples_demo();
    points_with_tuple();
}
