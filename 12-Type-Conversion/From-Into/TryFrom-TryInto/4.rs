// Similar to From and Into, TryFrom and TryInto are generic traits for converting between types.
// Unlike From/Into, TryFrom and TryInto are used for fallible conversions and return a Result
// instead of a plain value.

// Given

// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::TryInto;
// fn main() {
//     let n: i16 = 256;

//     // Into trait has a method `into`,
//     // hence TryInto has a method ?
//     let n: u8 = match n.__() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("there is an error when converting: {:?}, but we catch it", e.to_string());
//             0
//         }
//     };

//     assert_eq!(n, __);

//     println!("Success!")
//}

// My Solution
fn main() {
    let n: i16 = 256;

    use std::convert::TryInto; // Import TryInto into Scope

    let n: u8 = match n.try_into() { // Use .try_into()
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0); // 0 because the conversion should give an overflowing literal error

    println!("Success!")
}