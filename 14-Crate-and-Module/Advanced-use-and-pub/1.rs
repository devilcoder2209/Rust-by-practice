// We can bring two types of the same name into the same scope with use, but you need as
// keyword.

// Given

// use std::fmt::Result;
// use std::io::Result;

// fn main() {}

// My Solution

use std::fmt::Result;
use std::io::Result as IoResult; // use as IoResult to differentiate it from other result type

fn main() {}