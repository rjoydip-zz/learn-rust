fn main() {

    let x = 1;
    let y = 3;

    match (x, y) {
        (x,y) if x > y => { println!("Decreasing"); }
        (x,y) if y > x => { println!("Increasing"); }
        _              => { println!("Equal")}
    }
}
