fn main() {
    println!("Borrowing Demo");

    let v = vec![1, 2, 3, 4];
    println!("v = {:?}", v);

    let print_vec = |x: &Vec<i32>| {
        println!("inside closure => vec = {:?}", x);

        // cannot do the following as we cannot borrow vec as mutable!
        //x.push(66);
    };

    // -------------------------------------------------------------------

    print_vec(&v);
    println!("after closure execution => v = {:?}", v);

    let mut v_mut = vec![4, 5, 6];
    println!("v_mut = {:?}", v_mut);

    let mut_vector = |x: &mut Vec<i32>| {
        x.push(25);
        for i in 2..10 {
            x.push(i * 5);
        }
    };

    mut_vector(&mut v_mut);
    println!("after closure execution => v_mut = {:?}", v_mut);

    // -------------------------------------------------------------------

    let mut pair = (1, 2);
    println!("before => pair = ({},{})", pair.0, pair.1);
    
    { // scope starts
        let pair_ref = &mut pair;
        pair_ref.0 += 20;
        println!("inside closure => pair_ref = ({},{})", pair_ref.0, pair_ref.1);
    } // scope ends

    println!("after => pair = ({},{})", pair.0, pair.1);

    let mut a = 90;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);
}

// Output:
/*
Borrowing Demo
v = [1, 2, 3, 4]
inside closure => vec = [1, 2, 3, 4]
after closure execution => v = [1, 2, 3, 4]
v_mut = [4, 5, 6]
after closure execution => v_mut = [4, 5, 6, 25, 10, 15, 20, 25, 30, 35, 40, 45]
before => pair = (1,2)
inside closure => pair_ref = (21,2)
after => pair = (21,2)
a = 92
*/
