fn main() {
    // split a siting
    let _str = "This is a simple string";
    let (first, second) = _str.split_at(3);
    println!("First: {} | second: {}", first, second);
}
