// By default, you can pad string with spaces

// Given

// fn main() {
//     // the following two are padding with 5 spaces
//     println!("Hello {:5}!", "x"); // =>  "Hello x    !"  
//     println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

//     /* Fill in the blanks */
//     assert_eq!(format!("Hello __!", 5, "x"), "Hello x    !");
//     assert_eq!(format!("Hello __!", "x", width = 5), "Hello x    !");

//     println!("Success!")
// }

// My Solution
fn main() {
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"  
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    assert_eq!(format!("Hello {1:0$}!", 5, "x"), "Hello x    !"); // Pad x with width '5'
    assert_eq!(format!("Hello {0:width$}!", "x", width = 5), "Hello x    !"); // Pad x with amount 'width' 

    println!("Success!")
}
