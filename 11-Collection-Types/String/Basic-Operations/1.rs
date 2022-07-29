// Given

// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Dont't add/remove any code line
// fn main() {
//     let mut s: String = "hello, ";
//     s.push_str("world".to_string());
//     s.push(__);

//     move_ownership(s);

//     assert_eq!(s, "hello, world!");

//     println!("Success!")
// }

// fn move_ownership(s: String) {
//     println!("ownership of \"{}\" is moved here!", s)
// }

// My Solution
fn main() {
    let mut s: String = String::from("hello, "); // Use String::from() to convert the string literal to a String
    s.push_str("world"); // Remove to_string() because push_str expects a string literal
    s.push('!'); // .push() only adds a char so we give ! as a parameter

    move_ownership(s.clone()); // Cloning s so the ownership of s doesn't move and we can use it in assert_eq!

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) { 
    println!("ownership of \"{}\" is moved here!", s)
}