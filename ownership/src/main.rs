fn main() {
    // memory is known at compile time and created in stack
    let string_literal = "hello";

    // memory is known at run time and created in heap
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // after s2 = s1 the s1 is released and no longer valid
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // will be an error

    // copy heap data and stack data use clone()
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // functions and ownership
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
