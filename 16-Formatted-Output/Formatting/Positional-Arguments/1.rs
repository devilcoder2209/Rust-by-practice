// Given

/* Fill in the blanks */
// fn main() {
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");// => Alice, this is Bob. Bob, this is Alice
//     assert_eq!(format!("{1}{0}", 1, 2), __);
//     assert_eq!(format!(__, 1, 2), "2112");
//     println!("Success!");
// }

// My Solution
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    assert_eq!(format!("{1}{0}", 1, 2), "21"); // 21 because index starts at 0
    assert_eq!(format!("{1}{0}{0}{1}", 1, 2), "2112"); // Just make assert work
    println!("Success!");
}