// To convert any type to String, you can simple use the Tostring trait for that type. Rather than doing 
// that directly, you should implement the fmt::Display trait which automatically provides
// ToString ans also allows you to print the type with println!.

// Given
// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     // IMPLEMENT fmt method
// }

// fn main() {
//     let origin = Point { x: 0, y: 0 };
//     // FILL in the blanks
//     assert_eq!(origin.__, "The point is (0, 0)");
//     assert_eq!(format!(__), "The point is (0, 0)");

//     println!("Success!")
// }

// My Solution
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // Make format method that takes a mutable reference to the formatter 
        write!(f, "The point is ({}, {})", self.x, self.y) // Write the string literal to the default formatter
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");  // Use to_string() method
    assert_eq!(format!("{}", origin), "The point is (0, 0)"); // Just format a string to match the other one or convert origin to string

    println!("Success!")
}