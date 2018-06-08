fn use_slice(slice: &mut [i32])
{
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 6654;
}

fn slices_demo()
{
    let mut data = [1,2,3,4,5];
    println!("original data = {:?}", data);

    use_slice(&mut data[2..4]);

    println!("data after use_slice call = {:?}", data);
}

fn main()
{
    println!("Slices demo");
    slices_demo();
}
