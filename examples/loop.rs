fn main() {
    // loop forever
    loop {
        println!("Forever");
    }

    // while loop
    let mut count = 5;
    while count > 1 {
        count -= 1;
        println!("Count is {}", count);
    }

    // for loop
    let arr = [10, 20, 30, 40, 50];
    for ele in arr.iter() {
        println!("Element of array is {}", ele);
    }

    // loop reversed array 
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
