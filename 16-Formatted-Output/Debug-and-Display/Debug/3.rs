// We can also manually implement Debug trait for our types

// Given
// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);


// fn main() {    
//     // The problem with `derive` is there is no control over how
//     // the results look. What if I want this to just show a `7`?

//     /* Make it print: Now 7 will print! */
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }

// My Solution
use std::fmt; // Put fmt into scope

#[derive(Debug)]
struct Structure(i32);

// #[derive(Debug)] // Remove Debug implementation
struct Deep(Structure);

impl std::fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.0) // We write self.0.0 as 0 is the first element of the Struct Deep and since Deep takes in another struct Structure, we get the first element of it too
    }
}


fn main() {
    println!("Now {:?} will print!", Deep(Structure(7)));
}