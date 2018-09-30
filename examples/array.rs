fn main() {
    let _arr: [i16; 5] = [1,2,3,4,5]; // restric array will maximum of element and also i16

    // array with all 1
    let _arr = [1; 5];

    for _ele in _arr.iter() {
        println!("Array element is {}", _ele);
    }

    // mutable array
    let mut _arr = [1; 5];
    _arr[1] = 2;

    for _ele in _arr.iter() {
        println!("Array element is {}", _ele);
    }

    // length of the array
    let _arr = [1; 5];
    println!("Array length is {}", _arr.len());
}
