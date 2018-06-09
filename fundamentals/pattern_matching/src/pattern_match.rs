// suppose we have a function to query the job title based on the number of years of work experience in a company
fn query_title(x: i32) -> &'static str {
    match x {
        0 => "new hire",
        1 => "probatory",
        2 | 3 => "junior engineer",
        _z @ 4...5 => "senior engineer",
        _z @ 6...9 => "staff engineer",
        _z if (x >= 10) => "principal engineer",
        _ => "invalid",
    }
}

fn pattern_match_with_number_to_string() {
    for x in 0..15 {
        println!("Years of experience = {}, Title = {}", x, query_title(x));
    }
}

// function taking a tuple as an argument
fn query_points(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origin",
        (0, _) => "x-axis",
        (_, 0) => "y-axis",
        (_, _) => "any x and y",
    }
}

fn pattern_match_with_tuple() {
    let point = (5, 4);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis x = {}", x),
        (_, y) => println!("(?,{})", y),
    }

    // passing tuple to a function
    let origin = (0, 0);
    println!("point = {:?} => {}", origin, query_points(origin));

    let x_axis = (0, 9);
    println!("point = {:?} => {}", x_axis, query_points(x_axis));

    let y_axis = (9, 0);
    println!("point = {:?} => {}", y_axis, query_points(y_axis));

    let any_point = point;
    println!("point = {:?} => {}", any_point, query_points(any_point));
}

pub fn pattern_matching_demo() {
    pattern_match_with_number_to_string();
    pattern_match_with_tuple();
}
