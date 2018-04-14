fn main() {
    // print a variable
    let x = 5;
    println!("{}", x);

    // variable mutant
    let mut x = 5;
    x = 9;
    println!("{}", x);

    // print multiple variable
    let (x, y): (i32, i16) = (15, 16);
    println!("{}", x);
    println!("{}", y);
    println!("Multiple value print ({}, {})", x, y);
}