fn main() {
    let x = 10;

    if x == 1 {
        println!("Inside if");
    } else if x > 5{
        println!("Inside else if");
    } else {
        println!("Inside else");
    }

    // if else with let
    let condition = false;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

main();