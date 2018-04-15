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