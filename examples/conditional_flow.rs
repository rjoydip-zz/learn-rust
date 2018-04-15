fn main() {
    // simple conditional flow
    let x = 10;

    if x == 1 {
        println!("Inside if");
    } else if x > 5{
        println!("Inside else if");
    } else {
        println!("Inside else");
    }

    // if in a let
    let condition = false;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
