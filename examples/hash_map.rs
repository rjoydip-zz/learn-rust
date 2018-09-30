use std::collections::HashMap;

fn main() {
    /**
     * Example 1
     * Simple hash map
     * Insert value in HashMap and print that
     **/
    let mut hm = HashMap::new();
    hm.insert(String::from("hello"), 1);
    hm.insert(String::from("world"), 2);
    for (k, v) in &hm {
        println!("Key: {} and Value: {}", k, v);
    }

    // get value from HashMap
    match hm.get(&String::from("world")) {
        Some(&n) => println!("{}", n),
        None => println!("No match"),
    }

    // remove from HashMap

    hm.remove(&String::from("hello"));
}
main();