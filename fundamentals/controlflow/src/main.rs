fn if_demo()
{
    let day = "Saturday";

    if day == "Saturday"
    {
        println!("It's a holiday!");
    }
    else {
        println!("It's a work day!");
    }

    // if as an expression
    let age = 12;
    let group = if age < 3 {"toddler"} 
                else if age > 3 && age <= 12 {"child"} 
                else { "adult"};
    println!("age group = {}", group);
}

fn while_demo()
{
    let mut x = 1;
    while x < 100 {
        x *= 3;
        if x == 81 {
            continue;
        }
        println!("x = {}", x);
    }
}

fn loop_demo()
{
    println!("loop_demo");

    let mut x = 1;
    println!("Powers of 2");
    loop {
        x *= 2;
        println!("x = {}", x);

        if x == 1<<10 {
            break;
        }
    }

    let mut i = 1;
    loop {
        println!("1<<{} = {}", i, 1<<i);
        i += 1;
        if i == 11 {
            break;
        }
    }
}

fn for_demo()
{
    println!("for_demo");

    // prints from 0 to 9
    for i in 0..10 {
        println!(" i = {}", i);
    }

    println!("for_demo with positional values");

    // printing the position value also (prints from 20 to 39)
    for (pos, x) in (20..40).enumerate() {
        println!("pos = {} value = {}", pos,x);
    }
}

fn match_demo()
{
    println!("match_demo");
    
    let error_code = 2;
    let error_msg = match error_code {
        0 => "General error",
        1 => "File read error",
        2 => "Network error",
        5 => "Insufficient memory error",
        401 => "Internet connection error",
        500...1000 => "Unknown error", // notice three ...
        _ => "Invalid error code" // for all other cases not covered above
    };

    println!("Error code = {} Error message = {}", error_code, error_msg);
}

fn main() {
    println!("Control flow demo\n");

    if_demo();
    while_demo();
    loop_demo();
    for_demo();
    match_demo();
}
