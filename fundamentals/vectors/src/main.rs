fn vectors_demo()
{
    let mut v = vec![5,6,7,8];
    println!("{:?}", v);

    v.push(10);
    println!("{:?}", v);

    // another way of creating vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(500);
    v1.push(600);
    v1.push(700);
    println!("{:?}", v1);

    println!("v1[0] = {}", v1[0]);
    let idx:usize = 0; // will not work with :i32
    println!("v1[idx {}] = {}", idx, v1[idx]);

    // println!("unsafe access");
    // println!("v1[10] = {}", v1[10]);// will get panic->index out of bounds

    println!("safe access");
    match v1.get(10) {
        Some(x) => println!("v1[10] = {}", x),
        None => println!("Index out of bound!")
    }
}

fn iterate_vector()
{
    println!("iterating vector ...");
    let mut v: Vec<i32> = Vec::new();
    for i in 30..40 {
        v.push(i*2);
    }
    println!("{:?}", v);

    println!("contents of v: ");
    for elem in &v {
        println!("{}", elem);
    }

    println!("popping last element...");
    let last_elem = v.pop();
    println!("last_elem = {:?}, v = {:?}", last_elem, v);

    println!("printing v in reverse order ...");
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn main() 
{
    println!("vectors demo");
    vectors_demo();
    iterate_vector();
}
