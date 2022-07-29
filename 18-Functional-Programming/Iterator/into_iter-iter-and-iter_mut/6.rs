// Given

/* Fill in the blank */
// fn main() {
//     let mut values = vec![1, 2, 3];
//     let mut values_iter = values.__;

//     if let Some(v) = values_iter.__{
//         __
//     }

//     assert_eq!(values, vec![0, 2, 3]);
// }

// My Solution
/* Fill in the blank */
fn main() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut(); // Make a mutable iterator because we need to modify first value to 0

    if let Some(v) = values_iter.next() { // Use .next() to get the first value
        *v = 0
    }

    assert_eq!(values, vec![0, 2, 3]);
}
