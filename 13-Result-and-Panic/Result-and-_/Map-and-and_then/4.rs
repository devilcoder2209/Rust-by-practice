// map and and_then are two common combinators for Result<T, E> (also for Option<T>).

// Given
// use std::num::ParseIntError;

// // FILL in the blank in two ways: map, and then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
//    n_str.parse::<i32>().__
// }

// fn main() {
//     assert_eq!(add_two("4").unwrap(), 6);

//     println!("Success!")
// }

// My Solution
use std::num::ParseIntError;

// FILL in the blank in two ways: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    // Way One
    // Use map with a closure that returns the num + 2
    // n_str.parse::<i32>().map(|x| { x + 2 })

    // Way Two
    // Use and_then with a closure to return the num + 2
    n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}
