// Floating point precision

// Given

/* Fill in the blanks */
// fn main() {
//     let v = 3.1415926;

//     println!("{:.1$}", v, 4); // same as {:.4} => 3.1416 

//     assert_eq!(format!("__", v), "3.14");
//     assert_eq!(format!("__", v), "+3.14");
//     assert_eq!(format!("__", v), "3");

//     println!("Success!")
// }

// My Solution
fn main() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416 

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");

    println!("Success!")
}
