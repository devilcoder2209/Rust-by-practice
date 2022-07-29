// Given

// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     // IMPLEMENT `from` method
// }

// // FILL in the blanks
// fn main() {
//     let num = __(30);
//     assert_eq!(num.value, 30);

//     let num: Number = __;
//     assert_eq!(num.value, 30);

//     println!("Success!")
// }

// Given
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(num: i32) -> Number { // Implement a method that returns a Number instance
        Number {
            value: num,
        }
    }
}

// FILL in the blanks
fn main() {
    let num = Number::from(30); // Use from trait to get a Number from 30
    assert_eq!(num.value, 30);

    let num: Number = Number {value: 30}; // Make a nNumber instance manually
    assert_eq!(num.value, 30);

    println!("Success!")
}