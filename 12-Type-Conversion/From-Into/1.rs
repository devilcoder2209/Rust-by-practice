// The From trait allows for a type to define how to create itself from another type, hence providing a
// very simple mechanism for converting between several types.
// The From and Into traits are inherently linked, and this is actually part of its implementation. It 
// means if we write something like this: impl From<T> for U, then we can use let u: U = U::from(T) or
// let u: U = T.into().
// The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From
// trait for your type, then the Into trait will be automatically implemented for the same type.
// Using the Into trait will typically require the type annotations as the compiler is unable to 
// determine this most of the time.
// For example we can easily convert &str into String because the standard library has already implemented
// this for us: impl From<&`_ str> for String.

// Given

// fn main() {
//     // impl From<bool> for i32
//    let i1:i32 = false.into();
//    let i2:i32 = i32::from(false);  
//    assert_eq!(i1, i2);
//    assert_eq!(i1, 0);

//    // FIX the error in two ways
//    // 1. impl From<char> for ? , maybe you should check the docs mentiond above to find the answer
//    // 2. a keyword from the last chapter
//    let i3: i32 = 'a'.into();

//    // FIX the error in two ways
//    let s: String = 'a' as String;

//    println!("Success!")
// }

// My Solution
fn main() {
    // impl From<bool> for i32
   let i1:i32 = false.into();
   let i2:i32 = i32::from(false);  
   assert_eq!(i1, i2);
   assert_eq!(i1, 0);

   // Way one
   // Use as keyword
   let i3: i32 = 'a' as i32;

   // Way two
   // Use .try_into()
   use std::convert::TryInto;
   let i3: u32 = 'a'.try_into().unwrap();

   // Way three
   // Use .into()
   let i3: u32 = 'a'.into();

   // Way One
   // Use String::from
   let s: String = String::from('a');

   // Way two
   // Use .into()
   let s: String = 'a'.into();

   println!("Success!")
}