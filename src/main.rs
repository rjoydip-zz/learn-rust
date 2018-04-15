
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = 10;
    let b = &a;
    let ref c = a;

    println!("A: {}, B: {}, C: {}", a, b, c);
    if b == c {
        println!("B and C are equal");
    }
}