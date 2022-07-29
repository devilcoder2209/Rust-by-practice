// By default, overflow will cause compile errors, but we can add an global annotation to
// suppress these errors.

// Given

// fn main() {
//     assert_eq!(u8::MAX, 255);
//     // the max of `u8` is 255 as shown above.
//     // so the below code will cause an overflow error: literal out of range for `u8`.
//     // PLEASE looking for clues within compile errors to FIX it.
//     // DON'T modify any code in main.
//     let v = 1000 as u8;

//     println!("Success!")
// }

// My Solution
#[allow(overflowing_literals)] // This will will let the program compile even if there are overflowing literals
fn main() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;

    println!("Success!")
}