fn main() {
    let mut _arr = [1; 5];
    _arr[1] = 2;

    for _ele in _arr.iter() {
        println!("Array element is {}", _ele);
    }
}
