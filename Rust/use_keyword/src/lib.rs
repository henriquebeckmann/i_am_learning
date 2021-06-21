#![allow(unused)]
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
use std::{cmp::Ordering, io};
//use std::io::{self, Write}; // commented because the name `io` has already been defined
use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
