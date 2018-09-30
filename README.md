# Learn Rust

Learn rust with simple examples.

## Requirement

- [Rust](https://github.com/rust-lang/rust)
  `Rust` is the main source code repository for [Rust](https://www.rust-lang.org/en-US/). It contains the compiler, standard library, and documentation.
  - [Installation](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
  - [The Book](https://doc.rust-lang.org/book/index.html).
- [Cargo](https://github.com/rust-lang/cargo)
  `Cargo` is distributed by default with Rust, so if you've got rustc installed locally you probably also have cargo installed locally.

## Installation

```sh
git clone https://github.com/rjoydip/learn-rust.git
cd learn-rust
npm start
```

## Table of contents

- [Hello rust](#hello-rust)
- [Comments](#comments)
- [Variables](#variables)
- [Primitive data types](#primitive-data-types)
- [Conditional flow](#conditional-flow)
  - [If else](#if-else)
  - [If else with let](#if-else-with-let)
- [Casting](#casting)
- [Array](#array)
- [Tuple](#tuple)
- [Loop](#loop)
  - [Infinite](#infinite)
  - [While](#while)
  - [For](#for)
  - [Reverse](#reverse)
  - [Enumerate](#enumerate)
  - [Break](#break)
  - [Continue](#continue)
- [String](#string)
  - [Split](#split)
  - [Slice](#slice)
  - [Concat](#concat)
- [Match](#match)
- [Reference](#reference)
- [Struct](#struct)
- [Implement](#implement)
- [Enum](#enums)
- [Related function](#related-function)
- [Vector enum](#vector-enum)
- [Char match](#char-match)
- [Trait](#trait)
- [Owenership and Borrowing](#owenership_borrowing)

### Hello rust

```rs
println!("Hello, rust!");
```

### Comments

```rs
// single line comment

/*
    Multiline comments
*/
```

### Variables

```rs
// print a variable
let x = 5;
println!("{}", x);

// variable mutant
let mut x = 5;
x = 9;
println!("{}", x);

// print multiple variable
let (x, y): (i32, i16) = (15, 16);
println!("{}", x);
println!("{}", y);
println!("Multiple value print ({}, {})", x, y);

// print variable
let _x = 10;
println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", _x, _x, _x);

// display disimal value
println!("Display output 1: {:.2}", 1.234);
```

### Primitive data types

- **bool :** The boolean type.
- **char :** A character type.
- **i8 :** The 8-bit signed integer type.
- **i16 :** The 16-bit signed integer type.
- **i32 :** The 32-bit signed integer type.
- **i64 :** The 64-bit signed integer type.
- **isize :** The pointer-sized signed integer type.
- **u8 :** The 8-bit unsigned integer type.
- **u16 :** The 16-bit unsigned integer type.
- **u32 :** The 32-bit unsigned integer type.
- **u64 :** The 64-bit unsigned integer type.
- **usize :** The pointer-sized unsigned integer type.
- **f32 :** The 32-bit floating point type.
- **f64 :** The 64-bit floating point type.
- **array :** A fixed-size array, denoted [T; N], for the element type, T, and the no-negative compile-time constant size, N.
- **slice :** A dynamically-sized view into a contiguous sequence, [T].
- **str :** String slices.
- **tuple :** A finite heterogeneous sequence, (T, U, ..).

### Conditional flow

#### If else

```rs
let x = 10;

if x == 1 {
    println!("Inside if");
} else if x > 5{
    println!("Inside else if");
} else {
    println!("Inside else");
}
```

#### If else with let

```rs
let condition = false;
let number = if condition {
    5
} else {
    6
};
println!("The value of number is: {}", number);
```

### Casting

```rs
let f = 24.4321_f32;
let i = f as u8;
println!("{}, {}", f, i);
```

### Array

An array is fixed-size, collection of same-type elements.

```rs
let array: [i32; 5] = [0, 1, 2, 3, 4];
println!("The first element of the array is: {}", array[0]);

// slice
let slice = &array[0..3];
```

### Tuple

Tuples are finite, heterogeneous, sequences. Let's unpack that quickly. First of all they are finite; this is fairly self-explanatory. They have a size, a fixed number of elements. They are heterogeneous. They can contain multiple different types. This is in contrast to an array, which can only contain elements of the same type. And lastly they are sequences, meaning they have an order, and most importanly they can be accessed by index (although in a different manner than arrays).

```rs
let tuple = ("hello", 42, "world", [3,6,9]);

println!("First element is {}", tuple.0);
println!("Second element is {}", tuple.1);
println!("Third element is {}", tuple.2);
let mut counter = 0;
for x in &tuple.3 {
    println!("Element {} of the fourth element is {}", counter, x);
    counter += 1;
}
```

### Loop

#### Infinite

```rs
loop {
    println!("Infinite");
}
```

#### While

```rs
let mut count = 5;
while count > 1 {
    count -= 1;
    println!("Count is {}", count);
}
```

#### For

```rs
let arr = [10, 20, 30, 40, 50];
for ele in arr.iter() {
    println!("Element of array is {}", ele);
}
```

#### Reverse

```rs
for number in (1..4).rev() {
    println!("{}!", number);
}
```

#### Enumerate

```rs
for (i, x) in (1..10).enumerate() {
    println!("index is: {} and value is: {}", i, x);
}
```

#### Break

```rs
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
```

#### Continue

```rs
for i in 1..20 {
    if i == 2 {
        continue;
    }
    println!("Value of i is: {}", i);
}
```

### String

#### split

```rs
let str = "This is a simple string";
let (first, second) = str.split_at(3);
println!("First: {} | second: {}", first, second);
```

#### slice

```rs
let str = String::from("String");
println!("{}", &str[2..4]);
```

#### concat

```rs
let str1 = String::from("String 1");
let str2 = String::from("String 2");
let concat = str1 + " and "+ &str2;
println!("{:?}", &concat);
```

### Match

```rs
// match with number
let x = 4;
match x {
    0 => { println!("Value is 0"); }
    1 => { println!("Value is 1"); }
    2 => { println!("Value is 2"); }
    3 => { println!("Value is 3"); }
    4 => { println!("Value is 4"); }
    5 => { println!("Value is 5"); }
    _ => { println!("Any value match"); }
}

// match with string
let x = '6';
match x {
    '0' => { println!("Value is 0"); }
    '1' => { println!("Value is 1"); }
    '2' => { println!("Value is 2"); }
    '3' => { println!("Value is 3"); }
    '4' => { println!("Value is 4"); }
    '5' => { println!("Value is 5"); }
    _ => { println!("Any value match"); }
}

// match with or
let x = 3;
match x {
    0|1|2 => { println!("Value is {}", x); }
    3 => { println!("Value is three"); }
    _ => { println!("Any value match"); }
}

// match with condition
let x = 1;
let y = 3;
match (x, y) {
    (x,y) if x > y => { println!("Decreasing"); }
    (x,y) if y > x => { println!("Increasing"); }
    _              => { println!("Equal")}
}

// match with ...
let a = 8;
match a {
    4...7 => println!("In between 4...7: {}", a),
    _ => println!("Not in between 4...7"),
}

// pair/tuple match
let pair = (1, 0);
match pair {
    (0, y) => println!("y: {}", y),
    (x, 0) => println!("x: {}", x),
    _ => println!("Not match"),
}

// pattern match
let p = 11;
match p {
    n @ 1 ... 10 => println!("Between 1 to 10 : {}", n),
    n @ 11 ... 20 => println!("Between 11 to 20 : {}", n),
    _ => println!("Not match"),
}
```

### Reference

```rs
let a = 10;
let b = &a;
let ref c = a;

println!("A: {}, B: {}, C: {}", a, b, c);
if b == c {
    println!("B and C are equal");
}
```

### Struct

```rs
// Example 1
struct Object {
    width: u32,
    height: u32,
}

fn area(obj: &Object) -> u32 {
    return obj.height * obj.width;
}

let o = Object {
    width: 3,
    height: 2
};

println!("{}x{} with area: {}", o.width, o.height, area(&o));

// Example 2
#[derive(Debug, Clone, Copy)]
struct A(i32);
#[derive(Debug)]
struct B(f32);

let a = A(1);
let b = B(1.1);
let c = a.clone();

println!("{:?}, {:?}, {:?}", a, b, c);
```

### Implement

```rs
struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn new(width: u32, height: u32) -> Object {
        Object {
            width: width,
            height: height,
        }
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

let o = Object {
    width: 3,
    height: 2
};
let obj = Object::new(o.width, o.height);
obj.show();
```

### Enum

```rs

enum Shape {
    Rectangle { height: u32, width: u32},
    Square(u32),
    Circle(f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { height, width } => (height * width) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}

let r = Shape::Rectangle { height: 10, width: 10 };
let s = Shape::Square(10);
let c = Shape::Circle(4.5);

println!("Area of rectangle: {}", r.area());
println!("Area of square: {}", s.area());
println!("Area of circle: {}", c.area());
```

### Related function

```rs
// Struct
struct Object {
    width: u32,
    height: u32,
}

// Methods
impl Object {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

// Related Function
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object {
            width: width,
            height: height,
        }
    }
}

let o = Object {
    width: 3,
    height: 2
};
let obj = Object::new(o.width, o.height);
obj.show();
```

### Vector enum

```rs
#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Test(String),
}
let r = vec![
    Example::Int(111),
    Example::Float(11.1),
    Example::Test(String::from("Hello world")),
];
println!("{:?}", r);
```

### Char match

```rs
let s = Some('c');

// Example 1
match s {
    Some(i) => println!("Value inside match: {}", i),
    None => {},
}

// Example 2
if let Some(i) = s {
    println!("Value inside if: {}", i);
} else {
    println!("Inside else");
}
```

### Trait

```rs
trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    height: u32,
    width: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.14 * self.radius * self.radius) as u32
    }
}

let c = Circle { radius: 100.2 };
let r = Rectangle { height: 2, width: 3 };
println!("Circle area: {}, Rectangle area: {}", c.area(), r.area());
```

### Owenership and Borrowing

#### Example 1

```rs
let x = 1; // x is owener of one and it stores in stack

let s = String::from("String");
let y = s; // moved refrence s owenership to y
// NOTE: only one reference can own one pice of data
println!("{}", s);
```

#### Example 2

```rs
// example is called owenership moving
fn take(_v: Vec<i32>) {
    println!("We took v: {} and {}", _v[0], _v[5]);
}

let mut _v = Vec::new();
for i in 1..10 {
    _v.push(i);
}        
take(_v); // transfering _v owenership from main -> take
println!("Finihed");
```

#### Example 3

```rs
// example is called owenership copy
fn copy(a: i32, b: i32) {
    println!("Value of a and b is (inside copy fn): {}, {}",a, b);
}

let a = 1;
let b = 2;
copy(a, b);
// unallocated from main because a,b just copied to another fn 
println!("Value of a and b is (inside main fn): {}, {}", a, b);
```
#### Example 4

```rs
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

let v = vec![1,2,3,4,5];

borrow1(&v);
borrow2(&v);
borrow3(&v);

println!("Value of v (inside main fn): {:?}", v);
```

#### Example 5

```rs
// complx owenership and borrowing
fn count(v: &Vec<i32>, val: i32) -> usize {
    let res = v.into_iter().filter(|&&x| x == val).count();
    res
}

let v = vec![1,2,1,2,1,3,4,5,3];

for &i in &v {
    let c = count(&v, i); // c an &v are borrowed
    println!("{} is repeated {} times", i, c);
}
```

## Concepts

- [What Is Ownership?](https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html#what-is-ownership)

## Examples

All example area here. [Link](examples)

## License

MIT © [Joydip Roy](https://github.com/rjoydip)