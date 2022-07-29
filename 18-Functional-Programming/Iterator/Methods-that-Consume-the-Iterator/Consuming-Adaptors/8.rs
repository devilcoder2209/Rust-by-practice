// The Iterator trait has a number of methods with default implementations provided by the standard library.
// Some of these methods call the method next to use up the iterator, so they are called consuming adaptors.

// Given

/* Fill in the blank and fix the errors */
// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
//     let total = v1_iter.sum();

//     assert_eq!(total, __);

//     println!("{:?}, {:?}",v1, v1_iter);
// }

// My Solution
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
    let total: i32 = v1_iter.sum(); // Annotate the type

    assert_eq!(total, 6); // 1 + 2 + 3 = 6

    println!("{:?}",v1); // We have exhausted v1_iter so we can't use it now
}
