// Implementing fmt::Display for a structure whose elements must be handled sepearately is triky.
// The problem is each write! generates a fmt::Result which must be handled in the same place.
// Fortunately, Rust provides the ? operator to help us eliminate some unnecessary codes for dealing
// with fmt::Result

// Given

/* Make it work */
// use std::fmt; 

// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Extract the value using tuple indexing,
//         // and create a reference to `vec`.
//         let vec = &self.0;

//         write!(f, "[")?;

//         // Iterate over `v` in `vec` while enumerating the iteration
//         // count in `count`.
//         for (count, v) in vec.iter().enumerate() {
//             // For every element except the first, add a comma.
//             // Use the ? operator to return on errors.
//             if count != 0 { write!(f, ", ")?; }
//             write!(f, "{}", v)?;
//         }

//         // Close the opened bracket and return a fmt::Result value.
//         write!(f, "]")
//     }
// }

// fn main() {
//     let v = List(vec![1, 2, 3]);
//     assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
//     println!("Success!")
// }

// My Solution
use std::fmt; 

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ");
            }
            write!(f, "{}: {}", count, v); // We also write the index now
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
    println!("Success!")
}
