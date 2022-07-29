// You can pad numbers with extra zeros.

// Given

// fn main() {
//     println!("Hello {:5}!", 5); // => Hello     5!
//     println!("Hello {:+}!", 5); // =>  Hello +5!
//     println!("Hello {:05}!", 5); // => Hello 00005!
//     println!("Hello {:05}!", -5); // => Hello -0005!

//     /* Fill in the blank */
//     assert!(format!("{number:0>width$}", number=1, width=6) == __);
    
//     println!("Success!")
// }

// My Solution

fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
    
    println!("Success!")
}
