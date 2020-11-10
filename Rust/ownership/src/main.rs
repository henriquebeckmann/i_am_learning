#![allow(unused)]
fn main() {
    {                       // s is not valid here, it's not yet declared
        let s = "hello";    // s is valid from this point forward

        // do stuff with s
    }                       // this scope is now over, and s is no longer valid

    let mut s = String::from("hello"); // s comes into scope

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
    // longer valid

    let x = 5; // x comes into scope
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    takes_ownership(s); // s's value moves into the function...
                       // ... and so is no longer valid here

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    let s1 = String::from("hello");

    let (s2, len) = calculate_lenght(s1);

    println!("The lenght of '{}' is {}.", s2, len);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

  // s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here some_strong goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its
                                 // return value into the function
                                 // that calls it


    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_lenght(s: String) -> (String, usize) {
    let lenght = s.len(); // len() returns the lenght of a string

    (s, lenght)
}
