/**
 * Example 1
 **/
fn main() {
    let x = 1; // x is owener of one and it stores in stack

    let s = String::from("String");
    let y = s; // moved refrence s owenership to y
               // NOTE: only one reference can own one pice of data
    println!("{}", s);
}

/**
 * Example 2
 **/
// example is called owenership moving
fn take(_v: Vec<i32>) {
    println!("We took v: {} and {}", _v[0], _v[5]);
}

fn main() {
    let mut _v = Vec::new();
    for i in 1..10 {
        _v.push(i);
    }
    take(_v); // transfering _v owenership from main -> take
    println!("Finihed");
}

/**
 * Example 3
 **/
// example is called owenership copy
fn copy(a: i32, b: i32) {
    println!("Value of a and b is (inside copy fn): {}, {}", a, b);
}

/**
 * Example 4
 **/
// more complecated
// taking v paramater reference of v
fn borrow1(v: &Vec<i32>) {
    println!("Value of v in borrow1 fn is: {}", (*v)[0]);
}

fn borrow2(v: &Vec<i32>) {
    println!("Value of v in borrow2 fn is: {}", v[1]);
}

fn borrow3(v: &Vec<i32>) {
    println!("Value of v in borrow3 fn is: {}", &v[1]);
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    borrow1(&v);
    borrow2(&v);
    borrow3(&v);

    println!("Value of v (inside main fn): {:?}", v);
}

/**
 * Example 5
 **/
// complx owenership and borrowing
fn count(v: &Vec<i32>, val: i32) -> usize {
    let res = v.into_iter().filter(|&&x| x == val).count();
    res
}

fn main() {
    let a = 1;
    let b = 2;
    let v = vec![1, 2, 1, 2, 1, 3, 4, 5, 3];

    for &i in &v {
        let c = count(&v, i); // c an &v are borrowed
        println!("{} is repeated {} times", i, c);
    }
    
    copy(a, b);
    // unallocated from main because a,b just copied to another fn
    println!("Value of a and b is (inside main fn): {}, {}", a, b);
}
