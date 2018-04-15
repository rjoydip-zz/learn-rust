fn main() {
    let a = 8;
    match a {
        4...7 => println!("In between 4...7: {}", a),
        _ => println!("Not in between 4...7"),
    }
}