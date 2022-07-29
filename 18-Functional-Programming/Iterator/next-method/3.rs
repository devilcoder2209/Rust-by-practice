// All iterators implement a trait named Iterator that is defined in the standard library:
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }
// And we can call the next method on iterators directly.

// Given

/* Fill the blanks and fix the errors.
Using two ways if possible */
// fn main() {
//     let v1 = vec![1, 2];

//     assert_eq!(v1.next(), __);
//     assert_eq!(v1.next(), __);
//     assert_eq!(v1.next(), __);
// }

// My Solution
fn main() {
    // Way One
    let mut v1 = vec![1, 2].into_iter(); // Turn v1 into an iterateable and make it mutable

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}
