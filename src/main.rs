fn main() {
    // enumerate
    for i in 1..20 {
        if i == 2 {
            continue;
        }
        println!("Value of i is: {}", i);
    }
}
