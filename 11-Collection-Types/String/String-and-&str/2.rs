// A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid
// UTF-8 sequence. String is heap allocated, growable and not null terminated.
// &str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used
// to view into a String, just like &[T] is a view into Vec<T>

// Given

// FILL in the blanks
// fn main() {  
//     let mut s = String::from("hello, world");
 
//     let slice1: &str = __; // in two ways
//     assert_eq!(slice1, "hello, world");
 
//     let slice2 = __;
//     assert_eq!(slice2, "hello");
 
//     let slice3: __ = __; 
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");
 
//     println!("Success!")
//  }

// My Solution
fn main() {  
    let mut s = String::from("hello, world");
 
    let slice1: &str = &s; // Reference to String s to make it &str
    assert_eq!(slice1, "hello, world");
 
    let slice2 = &s[..5]; // Slice type from String s till hello
    assert_eq!(slice2, "hello");
 
    let mut slice3: String = "hello, world".to_string(); // Mutable String and use to_string() function to make the literal a String
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
 
    println!("Success!")
 }