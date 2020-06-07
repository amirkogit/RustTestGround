// Using a string
// To run: cargo run --bin string

fn main() {
    let mut s = String::new();
    s.push('H');
    s.push('i');
    println!("s: {}", s);

    // constructing string from a string slice (&str)
    let s = "Hello".to_string();
    println!("s: {}", s);

    let s = String::from("Helloo");
    println!("s: {}", s);

    // append string to each other
    let mut s = "Hello".to_string();
    s.push_str("world!!");
    println!("s: {}", s);

    // iterate over the characters
    for ch in "RustCrate".chars() {
        print!("{}.", ch);
    }
    println!();

    // split string into two
    let (first, second) = "RustLanguage".split_at(4);
    println!("first: {}, second: {}", first, second);

    // split on individual lines
    let haiku = "\
                she watches\n\
                satisfied after love\n\
                he lies\n\
                lookign up at nothing\n\
                ";
    println!("haiku: {}", haiku);

    for line in haiku.lines() {
        println!("\t{}.", line);
    }

    // split on substrings
    for s in "Never;Give;Up".split(';') {
        println!("{}", s);
    }

    // when splitted string is at the beginning or end, it will result in empty string
    let s:Vec<_> = "::Hi::There::".split("::").collect();
    println!("s: {:?}", s);

    // eliminate the empty strings at the end by using split_terminator
    let s:Vec<_> = "Mr.X.".split_terminator('.').collect();
    println!("s: {:?}", s);

    // char has few method's that can be used to split on
    for s in "I'm2fast4you".split(char::is_numeric) {
        println!("{}", s);
    }

    // split only a certain amount of times
    for s in "It's not your fault, it's mine".splitn(3, char::is_whitespace) {
        println!("{}", s);
    }

    // get only the substrings that match a pattern
    for c in "The Dark Knight Rises".matches(char::is_uppercase) {
        println!("{}", c);
    }

    // check if a string starts with something
    let saying = "The early bird gets the worm";
    let starts_with_the = saying.starts_with("The");
    println!("Does \"{}\" start with \"The\"? : {}", saying, starts_with_the);

    let starts_with_bird = saying.starts_with("bird");
    println!("Does \"{}\" start with \"bird\"? : {}", saying, starts_with_bird);

    // check if a string ends with something
    let ends_with_worm = saying.ends_with("worm");
    println!("ends with worm: {}", ends_with_worm);
    let ends_with_tom = saying.ends_with("tom");
    println!("ends with tom: {}", ends_with_tom);

    // check if string contains something somewhere
    let contains_bird = saying.contains("bird");
    println!("contains bird? {}", contains_bird);
    let contains_eagle = saying.contains("eagle");
    println!("contains eagle? {}", contains_eagle);

    // remove whitespace
    let white_spaces = "     Many white    spaces    ";
    println!("whites_spaces: {}", white_spaces);

    let s:Vec<_> = white_spaces.split(' ').collect();
    println!("trimmed: {:?}", s);

    // using split_whitespace instead
    let s:Vec<_> = white_spaces.split_whitespace().collect();
    println!("{:?}", s);

    // remove leading and trailing whitespace
    let username = "     penguin\n".trim();
    println!("username: {}", username);

    // remove only leading whitespace
    let username = "      penguin\ntest".trim_left();
    println!("username: {}", username);

    // remove only trailing whitespace
    let username = "    penguinn\n".trim_right();
    println!("username: {}", username);

    // -----------------------------
    // parse a string into another datatype

    let num = "23".parse::<i32>();
    println!("{:?}", num);

    if let Ok(num) = num {
        println!("{} * {} = {}", num, num, num * num);
    }

    // -------------------------------------
    // replace all occurences of a pattern
    let s = "My dad is the best dad";
    let new_s = s.replace("dad", "mom");
    println!("new_s: {}", new_s);

    // replace all characters with their lowercase
    let lowercase = s.to_lowercase();
    println!("lowercase: {}", lowercase);

    // replace all characters with their uppercase
    let uppercase = s.to_uppercase();
    println!("uppercase: {}", uppercase);

    // repeat a string
    let hello = "hello..^o^ ";
    println!("Repeated hello: {}", hello.repeat(3));
}