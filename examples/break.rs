fn main() {
    let mut x = 0;

    // break with loop
    loop {
        println!("Value of i is  {}", x);
        if x >= 10 {
            break;
        }

        x+= 1;
    }

    // break with while
    while x < 100 {
        println!("Value of i is  {}", x);
        if x >= 10 {
            break;
        }

        x+= 1;
    }

    // break with for
    for _ele in 1..100 {
        println!("Value of i is  {}", x);
        if x >= 10 {
            break;
        }

        x+= 1;
    }
}
