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

    // loop with enumerate
    for (i, x) in (1..10).enumerate() {
        println!("index is: {} and value is: {}", i, x);
    }

    /**
     * Break
     **/
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

    /**
     * Continue
     **/
    for i in 1..20 {
        if i == 2 {
            continue;
        }
        println!("Value of i is: {}", i);
    }
}
