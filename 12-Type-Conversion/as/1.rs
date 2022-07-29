// Rust provides no implicit type conversion(coercion) between primitivie types. But explicit type
// conversions can be performed using the as keyword.

// Given

// FIX the errors and FILL in the blank
// DON'T remove any code
// fn main() {
//     let decimal = 97.123_f32;

//     let integer: __ = decimal as u8;

//     let c1: char = decimal as char;
//     let c2 = integer as char;

//     assert_eq!(integer, 'b' as u8);

//     println!("Success!")
// }

// My Solution
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8; // The type is u8 since we are converting decimal to u8

    let c1: char = decimal as u8 as char; // First convert decimal to integer and then to char
    let c2 = integer as char;

    assert_eq!(integer + 1, 'b' as u8); // Add + 1 to integer to make the value 98 so that they both match. We could also do 'b' as u8 - 1 to make both 97

    println!("Success!")
}
