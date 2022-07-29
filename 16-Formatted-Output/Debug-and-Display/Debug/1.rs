// The implementation of Debug is very straightforward: All types can derive the std::fmt::Debug
// implementation. This is not true for std::fmt::Display which must be manually implemented.
// {:?} must be used to print out the type which has implemented the Debug trait.

// Given

/* Fill in the blanks and Fix the errors */
// struct Structure(i32);

// fn main() {
//     // Types in std and Rust have implemented the fmt::Debug trait
//     println!("__ months in a year.", 12);

//     println!("Now __ will print!", Structure(3));
// }

// My Solution
#[derive(Debug)] // Implement the Debug trait for Structure
struct Structure(i32);

fn main() {
    println!("{:?} months in a year.", 12); // Use {} or {:?} because primitive types implement the fmt::Display trait

    println!("Now {:?} will print!", Structure(3)); // Because we implemented the Debug trait for Strcuture, we use :? operator
}