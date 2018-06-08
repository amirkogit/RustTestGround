fn arrays_demo()
{
    let mut a = [1,2,3,4,5,6];
    println!("a has {} elements, first = {}", a.len(), a[0]);

    let a1:[i32;6] = [1,2,3,4,5,6];
    println!("a1 has {} elements, second = {}", a1.len(), a1[1]);

    a[0] = 10;
    println!("a[0] after update = {}", a[0]);

    //a1[0] = 100; // won't compile

    assert_eq!(a,[10,2,3,4,5,6]);

    if a != [1,2,3,4,5,6] // size must match
    {
        println!("arrays are not equal!");
    }

    // fill some array with 2s
    let b = [2u16; 10];
    println!("{:?}",b); // debug print
    assert_eq!(b.len(),10);
    assert_eq!(b[1],2);
    
    // print content of b using for loop
    for i in 0..b.len() {
        println!("{}",b[i]);
    }
}

fn multi_dim_arrays()
{
    println!("Multi dimensional array demo");
    let matrix:[[f32;3]; 3] = 
    [
        [1.0,0.0,0.0],
        [0.0,1.0,0.0],
        [0.0,0.0,1.0]
    ];
    println!("{:?}", matrix);

    // print all diagonal values
    println!("diagonal values");
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == j {
                println!("matrix[{}][{}] = {}", i, j, matrix[i][j]);
            }
        }
    }
}

fn main()
 {
    println!("Arrays demo");
    arrays_demo();
    multi_dim_arrays();
}
