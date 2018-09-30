fn main() {
    let s = Some('c');

    // match 1
    match s {
        Some(i) => println!("Value inside match: {}", i),
        None => {},
    }

    // match 2
    if let Some(i) = s {
        println!("Value inside if: {}", i);
    } else {
        println!("Inside else");
    }
}
