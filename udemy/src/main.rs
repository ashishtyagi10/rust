#[allow(dead_code)]
#[allow(unused_imports)]
use std::mem;

fn main() {
    println!("Hello, world!");
    let age = 32;
    println!("age is {}", age);
    let c = 12345678;
    println!("{} is taking byte {}", c, mem::size_of_val(&c));
    let d:isize = 1234567890123456789;
    println!("{} is taking byte {}", d, mem::size_of_val(&d));
}
