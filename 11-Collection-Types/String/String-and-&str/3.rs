// Given

// Question: how many heap allocations are happened here ?
// Your answer: 
// fn main() {  
//     // Create a String type based on `&str`
//     // the type of string literals is `&str`
//    let s: String = String::from("hello, world!");

//    // create a slice point to String `s`
//    let slice: &str = &s;

//    // create a String type based on the recently created slice
//    let s: String = slice.to_string();

//    assert_eq!(s, "hello, world!");

//    println!("Success!")
// }

// My Solution

// Question: how many heap allocations are happened here ?
// Your answer: 2 allocations happened. Once when we declared s and then when we redeclared s with slice
fn main() {  
    // Create a String type based on `&str`
    // the type of string literals is `&str`
   let s: String = String::from("hello, world!");

   // create a slice point to String `s`
   let slice: &str = &s;

   // create a String type based on the recently created slice
   let s: String = slice.to_string();

   assert_eq!(s, "hello, world!");

   println!("Success!")
}
