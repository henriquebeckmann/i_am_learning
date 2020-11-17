#![allow(unused)]
fn main() {
    let mut s = String::from("hello");

    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);

    change(&mut s);

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM

    //println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let reference_to_nothig = dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

//fn dangle() -> &String { // dangle returns a reference to a String
    //let s = String::from("hello"); // s is a new String

    //&s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn dangle() -> String {
    let s = String::from("hello");

    s
}
