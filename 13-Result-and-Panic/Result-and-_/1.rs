// Result<T> is an enum to describe possible errors. It has two variants:
// - Ok(T): a value T was found
// - Err(e): an error was found with a value e
// In short words, the expected outcome is Ok, while the unexpected outcome is Err.

// Given

// FILL in the blanks and FIX the errors
// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) -> __ {
//     let n1 = n1_str.parse::<i32>();
//     let n2 = n2_str.parse::<i32>();
//     Ok(n1.unwrap() * n2.unwrap())
// }

// fn main() {
//     let result = multiply("10", "2");
//     assert_eq!(result, __);

//     let result = multiply("t", "2");
//     assert_eq!(result.__, 8);

//     println!("Success!")
// }

// My Solution
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20)); // Since 10 and two are valid inputs, we get the Ok() variant

    let result = multiply("4", "2"); // Change to 4 inorder to get 8
    assert_eq!(result.unwrap(), 8); // Call .unwrap() to get value of Ok

    println!("Success!")
}
