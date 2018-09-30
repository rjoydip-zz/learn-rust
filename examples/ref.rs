fn main() {
    let a = 10;
    let b = &a;
    let ref c = a;

    println!("A: {}, B: {}, C: {}", a, b, c);
    if b == c {
        println!("B and C are equal");
    }
}
main();
