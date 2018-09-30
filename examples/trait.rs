trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    height: u32,
    width: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.14 * self.radius * self.radius) as u32
    }
}

fn main() {
    let c = Circle { radius: 100.2 };
    let r = Rectangle { height: 2, width: 3 };

    println!("Circle area: {}, Rectangle area: {}", c.area(), r.area());
}
main();