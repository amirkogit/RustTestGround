mod sh; // uses the module sh.rs

fn main()
{
    // access public method defined in the module sh.rs
    sh::stack_heap_demo();

    // cannot do this as get_origin() is defined as private
    //let p1 = sh::get_origin(); 

    // sh::Point2D is private and not accessible here
    // let d1 = sh::Point2D {x:5.1, y:8.2};
}
