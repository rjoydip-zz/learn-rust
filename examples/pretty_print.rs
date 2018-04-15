use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// Methods
impl Object {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

// Related Function
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object {
            width: width,
            height: height,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) and Area: {}", self.width, self.height, self.area())
    }
}

// main fn
fn main() {
    let o = Object {
        width: 3,
        height: 2
    };
    let obj = Object::new(o.width, o.height);
    obj.show();
}