fn main() {
    // simple match
    let x = 4;
    match x {
        0 => { println!("Value is 0"); }
        1 => { println!("Value is 1"); }
        2 => { println!("Value is 2"); }
        3 => { println!("Value is 3"); }
        4 => { println!("Value is 4"); }
        5 => { println!("Value is 5"); }
        _ => { println!("Any value match"); }
    }
}
