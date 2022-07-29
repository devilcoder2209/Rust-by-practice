// Given

/* Refactoring the following code using iterators */
// fn main() {
//     let arr = [0; 10];
//     for i in 0..arr.len() {
//         println!("{}",arr[i])
//     }
// }

// My Solution
fn main() {
    let arr = [0; 10];
    for i in arr { // Use arr only
        println!("{}",arr[i])
    }
}
