// Returning closure is much harder than you may have thought of.

// Given

/* Fill in the blank using two approches,
 and fix the errror */
//  fn create_fn() -> __ {
//     let num = 5;

//     // how does the following closure capture the evironment variable `num`
//     // &T, &mut T, T ?
//     |x| x + num
// }


// fn main() {
//     let fn_plain = create_fn();
//     fn_plain(1);
// }

// My Solution

// Way One
// Use impl keyword
// fn create_fn() -> impl FnOnce(i32) -> i32 { // Use impl keyword to specify closure as return type
//     let num = 5;

//     move |x| x + num // Add move keyword
// }

// Way Two
// Use Box<>
fn create_fn() -> Box<dyn Fn(i32) -> i32> { // Return type is box ( see line 36 )
    let num = 5;

    // Pack the closure in Box
    Box::new(move |x| x + num)
}


fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
