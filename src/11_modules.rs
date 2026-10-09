
pub mod namespace {
    pub mod utility {
        pub fn doSomething() {
            
        } 
    }
}
fn main() {
    crate::namespace::utility::doSomething(); //Absolute Path Syntax
    namespace::utility::doSomething(); //Relative Path Syntax
}
mod superExampleMod {
    super::namespace::utility::doSomething(); //Super Syntax
}

use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
}
fn function2() -> IoResult<()> {
}
// You can rename imports with custom alias.

use std::cmp::Ordering;
use std::io;
//
use std::{cmp::Ordering, io};
//  Import shorthand

use std::io;
use std::io::Write;
//
use std::io::{self, Write};
//  Unique trick

use std::collections::*;
//  Wildcard example

//crate::front_of_house::hosting
// File system is:
// src/
//   front_of_house.rs
//   front_of_house/
//     hosting.rs