// A Vec can be mutable. ON the other hand, slices are read-only objects.
// To get a slice, use &.
// In Rust, it's more common to pass slices as arguments rather than vectors
// when you just want to provide rea access. The same goes for String and &str.

// Given

// FIX the errors
// fn main() {
//     let mut v = vec![1, 2, 3];

//     let slice1 = &v[..];
//     // out of bounds will cause a panic
//     // You must use `v.len` here
//     let slice2 = &v[0..4];
    
//     assert_eq!(slice1, slice2);
    
//     // slice are read only
//     // Note: slice and &Vec are different
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[0..3];
//     slice3.push(4);

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!")
// }

// My Solution
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];

    let slice2 = &v[0..v.len()]; // Use v.len() to get the length of the Vec v, we could also leave the end of the range which is v.len() too. so [0..] is also valid, and since we are beginning at the beginning, we can leave the start  of the range too. So [..] is a better solution
    
    assert_eq!(slice1, slice2);
    
    // slice are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..]; // Make the slice include 4. => make the range till the end of the Vector

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}

