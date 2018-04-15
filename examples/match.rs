fn main() {
    // match with number
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

    // match with string
    let x = '6';
    match x {
        '0' => { println!("Value is 0"); }
        '1' => { println!("Value is 1"); }
        '2' => { println!("Value is 2"); }
        '3' => { println!("Value is 3"); }
        '4' => { println!("Value is 4"); }
        '5' => { println!("Value is 5"); }
        _ => { println!("Any value match"); }
    }

    // match with or
    let x = 3;
    match x {
        0|1|2 => { println!("Value is {}", x); }
        3 => { println!("Value is three"); }
        _ => { println!("Any value match"); }
    }

    // match with condition
    let x = 1;
    let y = 3;
    match (x, y) {
        (x,y) if x > y => { println!("Decreasing"); }
        (x,y) if y > x => { println!("Increasing"); }
        _              => { println!("Equal")}
    }

    // match with ...
    let a = 8;
    match a {
        4...7 => println!("In between 4...7: {}", a),
        _ => println!("Not in between 4...7"),
    }

    // pair/tuple match
    let pair = (1, 0);
    match pair {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("Not match"),
    }

    // pattern match
    let p = 11;
    match p {
        n @ 1 ... 10 => println!("Between 1 to 10 : {}", n),
        n @ 11 ... 20 => println!("Between 11 to 20 : {}", n),
        _ => println!("Not match"),
    }
}
