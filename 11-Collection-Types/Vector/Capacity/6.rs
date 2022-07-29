// The capacity of a veccotr is the amount of space allocated for any future elements that will be added
// onto the vector. This is not to be confused with the length of a vecotr, which specifies the number of 
// actual elements within the vector. If a vector's length exceeds its capacity, its capacity will 
// automatically be increased, but its elements will have to be reallocated.
// For example, a vector with capacity 10 and length 0 would be an empty vector with space for 10
// more elements. Pushing 10 or fewer elements onto the cevtor will not change its capacity or cause 
// reallocation to occur. However, if the vector's length is increased to 11, it will have to reallocate 
// which can be slow. For this reason, it is recommended to use Vec::with_capacity wheneve possible 
// to specify how big the vector is expected to get.

// Given

// FIX the errors
// fn main() {
//     let mut vec = Vec::with_capacity(10);

//     // The vector contains no items, even though it has capacity for more
//     assert_eq!(vec.len(), __);
//     assert_eq!(vec.capacity(), 10);

//     // These are all done without reallocating...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), __);
//     assert_eq!(vec.capacity(), __);

//     // ...but this may make the vector reallocate
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);


//     // fill in an appropriate value to make the `for` done without reallocating 
//     let mut vec = Vec::with_capacity(__);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), __);
//     assert_eq!(vec.capacity(), __);
    
//     println!("Success!")
// }

// My Solution
fn main() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0); // Add 0 because the Vector is emmpty
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10); // Add 10 because the Vector has 10 elements now
    assert_eq!(vec.capacity(), 10); // Add 10 because the capacity is still the same

    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    let mut vec = Vec::with_capacity(100); // Add hundred as a capacity so it is enough for the element addition in the for loop
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100); // Add 100 because we have 100 elements now
    assert_eq!(vec.capacity(), 100); // Add 100 because the capacity is the same
    
    println!("Success!")
}
