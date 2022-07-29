// A function call with explicitly specified type parameters looks like: fun::<A, B, ...>().

// Given

// Implement the generic function below.
// fn sum

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }

// My Solution
fn sum<T: std::ops::Add + std::ops::Add<Output = T>>(num1: T, num2: T) -> T { // Make generic function, add traits to T
    num1 + num2 // Return sum
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");

}