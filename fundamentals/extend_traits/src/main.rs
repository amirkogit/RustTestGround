trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self {
            result += x;
        }
        return result;
    }
}

fn main() 
{
    println!("Extending Traits Demo");
    let a = vec![2,4,5,6];
    println!("a = {:?}", a);

    // If I want to get the sum of all elements of a vector like this:
    //let sum = a.sum(); // this won't compile until we extend a trait with this method!

    println!("sum(a) = {}", a.sum());

    let b = vec![5.2,6.3,5.0];
    //println!("sum = {}", b.sum()); // will give compile error!
    println!("b = {:?}", b);
}
