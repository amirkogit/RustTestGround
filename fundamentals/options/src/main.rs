fn options_demo()
{
    let x = 4.0;
    let y = 0.0;

    let result:Option<f64> = 
        if y != 0.0 { Some(x/y) } else { None };
    
    match result {
        Some(z) => println!("{}/{}={}",x,y,z),
        None => println!("Cannot divide {} by {}",x,y)
    }
}

fn main() {
    println!("Options demo");
    options_demo();
}
