fn strings_demo()
{
    // view into a string
    //&'static str is statically allocated
    let s:&'static str = "hello";
    println!("{}", s);

    let s ="new text"; // cannot reassign to s

    let mut s2 = "california";
    s2 = "washington";

    println!("s2 = {}", s2);

    println!("characters in reverse order...");
    for c in s2.chars().rev() {
        println!("{}", c);
    }

    // what if we want to get the nth character. this may return some char or nothing at all
    if let Some(first_char) = s2.chars().nth(0) {
        println!("first letter = {}", first_char);
    }

    let s3 = "sh";
    if let Some(third_char) = s3.chars().nth(3) {
        println!("third letter = {}", third_char);
    }
    else {
        println!("no third char. word is {}", s3);
    }
}

fn strings_heap_allocated_demo()
{   
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a = a + 1;
    }

    println!("letters: {}", letters)
}

fn strings_deference_demo()
{
    let mut letters = "abcdefg";
    let l:&str = &letters;
    println!("{}", l);
}

fn strings_concatenate_demo()
{
    let mut msg = "hello world".to_string();
    println!("{}", msg);
    msg.remove(0);
    println!("{}", msg);
    msg.push_str("!!!!");
    println!("{}", msg);
    println!("{}, {}", msg, msg.replace("ello", "*****"));
}

fn main() 
{
    println!("Strings demo");
    strings_demo();
    strings_heap_allocated_demo();
    strings_deference_demo();
    strings_concatenate_demo();
}
