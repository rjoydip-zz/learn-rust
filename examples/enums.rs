
enum Shape {
    Rectangle { height: u32, width: u32},
    Square(u32),
    Circle(f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { height, width } => (height * width) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}

fn main() {
    let r = Shape::Rectangle { height: 10, width: 10 };
    let s = Shape::Square(10);
    let c = Shape::Circle(4.5);

    println!("Area of rectangle: {}", r.area());
    println!("Area of square: {}", s.area());
    println!("Area of circle: {}", c.area());
}
