#![allow(dead_code)]
#![allow(unused_variables)]

struct Point2D {
    x: f64,
    y: f64
}

// not used in the program but we disabled the warning by using #![allow(dead_code)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64
}

struct Line2D {
    start: Point2D,
    end: Point2D
}

// not used in the program but we disabled the warning by using #![allow(dead_code)]
struct Line3D {
    start: Point3D,
    end: Point3D
}

impl Line2D 
{
    fn distance(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        return (dx*dx+dy*dy).sqrt();
    }
}

pub fn structures_demo()
{
    let p1 = Point2D { x: 5.0, y: 10.0};
    println!("p1 = ({},{})", p1.x, p1.y);

    let p2 = Point2D { x: 2.0, y: 6.0};
    println!("p2 = ({},{})", p2.x, p2.y);

    let line2d = Line2D { start: p1, end: p2 };
    println!("line2d start = ({},{}) line2d end = ({},{})", line2d.start.x, line2d.start.y, line2d.end.x, line2d.end.y);

    println!("line2d distance = {}",line2d.distance());
}