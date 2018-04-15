fn main() {
    // split a siting
    let _str = "This is a simple string";
    let (first, second) = _str.split_at(3);
    println!("First: {} | second: {}", first, second);

    // slice
    let _str = String::from("String");
    println!("{}", &_str[2..4]);

    // concat
    let _str1 = String::from("String 1");
    let _str2 = String::from("String 2");
    let _concat = _str1 + " and "+ &_str2;
    println!("{:?}", &_concat);
}
