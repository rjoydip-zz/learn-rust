// complx owenership and borrowing
fn count(v: &Vec<i32>, val: i32) -> usize {
    let res = v.into_iter().filter(|&&x| x == val).count();
    res
}

fn main() {
    let v = vec![1,2,1,2,1,3,4,5,3];

    for &i in &v {
        let c = count(&v, i); // c an &v are borrowed
        println!("{} is repeated {} times", i, c);
    }
}