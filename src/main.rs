#[derive(Debug, Clone, Copy)]
struct A(i32);
#[derive(Debug)]
struct B(f32);

fn main() {
    let a = A(1);
    let b = B(1.1);
    let c = a.clone();

    println!("{:?}, {:?}, {:?}", a, b, c);
}