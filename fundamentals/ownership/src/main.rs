fn main() {
    println!("Ownership demo");

    // here v1 is allocated on stack, however the data it points to is on heap
    let v1 = vec![1, 2, 3, 4];
    println!("v1 = {:?}", v1); // this is fine as no ownership has been transferred

    let v2 = v1; // makes a copy of data pointed by v1 to v2. Basically transfers the ownership from v1 to v2
    println!("v2 = {:?}", v2); // this is fine

    //  println!("{:?}", v1); // this will report error as v2 owns the data now!

    println!("v2 = {:?}", v2);

    let my_closure = |v:Vec<i32>| ();
    my_closure(v2);

    //println!("v2 = {:?}", v2); // reports error that value is used after move!

    let u1 = 2;
    let u2 = u1;

    // both would be fine in the following because i32 implements copy trait. so copy would be done instead of move
    println!("u1 = {}", u1);
    println!("u2 = {}", u2);

    let x1 = Box::new(5);
    println!("x1 = {}", x1);
    let x2 = x1;
    println!("x2 = {}", x2);
    //println!("x1 = {}", x1); // will give error as value is used here after move

    // giving the ownership back!
    let print_vec = |x:Vec<i32>| -> Vec<i32>
    {
        println!("inside closure => Vec<i32> = {:?}", x);
        x
    };

    let mut v3 = vec![1,3,7,13];
    println!("v3 = {:?}", v3);
    v3 = print_vec(v3);
    println!("after closure execution => v3 = {:?}", v3);
}
