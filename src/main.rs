fn main() {
    let p = 11;
    match p {
        n @ 1 ... 10 => println!("Between 1 to 10 : {}", n),
        n @ 11 ... 20 => println!("Between 11 to 20 : {}", n),
        _ => println!("Not match"),
    }
}