/**
 * Example 1
 **/

struct Object {
    width: u32,
    height: u32,
}

fn area(obj: &Object) -> u32 {
    // obj.height * obj.width
    return obj.height * obj.width;
}

fn main() {
    let o = Object {
        width: 3,
        height: 2
    };

    println!("{}x{} with area: {}", o.width, o.height, area(&o));
}

/**
 * Example 2
 **/
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
main();