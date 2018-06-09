// Suppose we want to find the sum of all even squares <= 500

fn is_even(x: u32) -> bool
{
    x % 2 == 0
}

fn use_naive_method()
{
    let limit = 500;
    let mut sum = 0;

    // 0.. Note that upper limit is not defined. We break when limit hits
    for i in 0.. {
        let sq = i * i;
        if sq > limit { break; }
        else if is_even(sq) {
            sum += sq;
        }
    }

    println!("naive method: sum = {}", sum);
}

fn use_higher_order_function()
{
    let limit = 500;
    let sum = 
        (0..).map(|x| x * x)
            .take_while(|&x| x < limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum,x| sum + x);
    println!("higher_order_function method: sum = {}", sum);
}

fn main() 
{
    println!("Higher Order Functions demo");
    use_naive_method();
    use_higher_order_function();
}
