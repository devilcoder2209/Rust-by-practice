// On a varaible by variable basis, the compiler will capture variables in the least restrictive manner
// possible.
// For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture
// by &T, &mut T, or T, but the compiler will ultimately choose based on how the captured variables
// are used in the closure. Which trait to use is determined by what the closure does with captured 
// value.
// This is because if a move is possible, then any type of borrow should be possible. Note that the
// reverse is not true. If the parameter is annotated as Fn, then capturing variables by &mut T or T
// are not allowed.

// Given

/* Fill in the blank */

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
// fn apply<F>(f: F) where
//     // The closure takes no input and returns nothing.
//     F: __ {

//     f();
// }

// // A function which takes a closure and returns an `i32`.
// fn apply_to_3<F>(f: F) -> i32 where
//     // The closure takes an `i32` and returns an `i32`.
//     F: Fn(i32) -> i32 {

//     f(3)
// }

// fn main() {
//     use std::mem;

//     let greeting = "hello";
//     // A non-copy type.
//     // `to_owned` creates owned data from borrowed one
//     let mut farewell = "goodbye".to_owned();

//     // Capture 2 variables: `greeting` by reference and
//     // `farewell` by value.
//     let diary = || {
//         // `greeting` is by reference: requires `Fn`.
//         println!("I said {}.", greeting);

//         // Mutation forces `farewell` to be captured by
//         // mutable reference. Now requires `FnMut`.
//         farewell.push_str("!!!");
//         println!("Then I screamed {}.", farewell);
//         println!("Now I can sleep. zzzzz");

//         // Manually calling drop forces `farewell` to
//         // be captured by value. Now requires `FnOnce`.
//         mem::drop(farewell);
//     };

//     // Call the function which applies the closure.
//     apply(diary);

//     // `double` satisfies `apply_to_3`'s trait bound
//     let double = |x| 2 * x;

//     println!("3 doubled: {}", apply_to_3(double));
// }

// My Solution

fn apply<F>(f: F) where
    F: FnOnce() {

    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
