struct Point2D
{
    x: f64,
    y: f64
}

struct Line2D
{
    start: Point2D,
    end: Point2D
}

// implements methods of Line2D struct
impl Line2D
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        let len = (dx * dx + dy * dy).sqrt();
        len
    }
}

fn methods_demo()
{
    let p1 = Point2D { x: 5.1, y: 6.3 };
    let p2 = Point2D { x: 8.6, y: 4.3 };
    let line = Line2D { start: p1, end: p2 };
    println!("length = {}", line.len());
}

fn main() 
{
    println!("Methods Demo");
    methods_demo();
}
