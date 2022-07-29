// We can use parse method to convert a String into a i32 number ,this is because 
// FromStr is implemented fro i32 type in standard  library: impl FromStr for i32

// Given

// To use `from_str` method, you needs to introduce this trait into the current scope.
// use std::str::FromStr;
// fn main() {
//     let parsed: i32 = "5".__.unwrap();
//     let turbo_parsed = "10".__.unwrap();
//     let from_str = __.unwrap();
//     let sum = parsed + turbo_parsed + from_str;
//     assert_eq!(sum, 35);

//     println!("Success!")
// }

// My Solution
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap(); // Use .parse() to get the int from a string
    let turbo_parsed = "10".parse::<i32>().unwrap(); // use .parse::<T>() to parse T from a String
    let from_str = i32::from_str("20").unwrap(); // Use i32.from_str() to convert a string to an integer
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}