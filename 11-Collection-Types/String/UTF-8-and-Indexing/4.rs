// Strings are always valid UTF-8. This has a few implications:
// - The first of which is taht if you need a non-UTF-8 string,
//   consider OsString. It is similar, but without the UTF-8 constraint.
// - The second implication is that you cannot index into a String
// Indexing is intended to be a constraint-time operation, but UTf-8 encoding
// does not allow us to do this. Furthermore, it's not clear what sort of thing the index
// should return: a byte, a codepoint, or a grapheme cluster. The bytes and chars mehtods
// return iterators over the first two, respectively

// You can't use index to access a char in a string, but you can use slice &s1[start..end].

// Given

// FILL in the blank and FIX errors
// fn main() {
//     let s = String::from("hello, 世界");
//     let slice1 = s[0]; //tips: `h` only takes 1 byte in UTF8 format
//     assert_eq!(slice1, "h");

//     let slice2 = &s[3..5];// tips: `中`  takes 3 bytes in UTF8 format
//     assert_eq!(slice2, "世");
    
//     // iterate all chars in s
//     for (i, c) in s.__ {
//         if i == 7 {
//             assert_eq!(c, '世')
//         }
//     }

//     println!("Success!")
// }

// My Solution
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[..=0]; // Add & to make it a slice and change 0 to an inclusive range ..=0
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // Change range to exclusive 7..10
    assert_eq!(slice2, "世");
    
    // iterate all chars in s
    for (i, c) in s.chars().enumerate() { // Add .chars() to get an iterateable of all the characters in the String and enumerate to get the index too
        if i == 7 { 
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}