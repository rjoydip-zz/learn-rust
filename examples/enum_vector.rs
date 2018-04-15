#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Test(String),
}

fn main() {
    let r = vec![
        Example::Int(111),
        Example::Float(11.1),
        Example::Test(String::from("Hello world")),
    ];

    println!("{:?}", r);
}