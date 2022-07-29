// Given

// FILL in the blanks
// use std::mem;

// fn main() {
//     let story = String::from("Rust By Practice");

//     // Prevent automatically dropping the String's data
//     let mut story = mem::ManuallyDrop::new(story);

//     let ptr = story.__();
//     let len = story.__();
//     let capacity = story.__();

//     assert_eq!(16, len);

//     // We can re-build a String out of ptr, len, and capacity. This is all
//     // unsafe because we are responsible for making sure the components are
//     // valid:
//     let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

//     assert_eq!(*story, s);

//     println!("Success!")
// }

// My Solution
use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr(); // Use as_mut_ptr to get the pointer as the String, story, is mutable. We would use as_ptr when story was immutable
    let len = story.len(); // Use len to get length of String
    let capacity = story.capacity(); // Use capacity to get the capacity of the buffer of the String

    assert_eq!(16, len);

    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}