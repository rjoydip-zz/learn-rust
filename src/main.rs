fn main() {
    let mut x = 0;

    for _ele in 1..100 {
        println!("Value of i is  {}", x);
        if x >= 10 {
            break;
        }

        x+= 1;
    }
}
