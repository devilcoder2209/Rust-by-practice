// For types that implement the Copy trait, like i32, the values are copied into HashMap. For owned
// values like String, the values will be moved and HashMap will be the owner of those values.

// Given

// FIX the errors with least changes
// DON'T remove any code line
// use std::collections::HashMap;
// fn main() {
//   let v1 = 10;
//   let mut m1 = HashMap::new();
//   m1.insert(v1, v1);
//   println!("v1 is still usable after inserting to hashmap : {}", v1);

//   let v2 = "hello".to_string();
//   let mut m2 = HashMap::new();
//   // ownership moved here
//   m2.insert(v2, v1);
    
//   assert_eq!(v2, "hello");

//   println!("Success!")
// }

// My Solution
use std::collections::HashMap;
fn main() {
  let v1 = 10;
  let mut m1 = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2 = "hello"; // Remove to_string() and only store &str 
  let mut m2 = HashMap::new();
  m2.insert(v2, v1);
    
  assert_eq!(v2, "hello");

  println!("Success!")
}