// ? is almost equivalent to unwrap, but ? returns instead of panic on Err.

// Given

// use std::num::ParseIntError;

// // IMPLEMENT multiply with ?
// // DON'T use unwrap here
// fn multiply(n1_str: &str, n2_str: &str) -> __ {
// }

// fn main() {
//     assert_eq!(multiply("3", "4").unwrap(), 12);
//     println!("Success!")
// }

// My Solution
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?; // Convert to int and add ? 
    let n2 = n2_str.parse::<i32>()?; // same here

    Ok(n1 * n2) // Return Ok variant with result if no errors encountered
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!")
}