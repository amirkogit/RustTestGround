fn scope_and_shadowing()
{
    println!("Scope and Shadowing demo");

    let a = 603;

    // new block 
    {
        println!("a = {}", a);

        let mut a = 1987;
        println!("a inside = {}", a);

        a = a + 2;
        println!("a inside incremented by 2 = {}",a);

        let b = 458;
        println!("b inside = {}", b);
        
    }

    //println!("b inside = {}", b); // cannot access b that is out of scope
}

fn main() 
{
    scope_and_shadowing();    
}
