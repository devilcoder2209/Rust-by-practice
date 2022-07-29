// Printing is handled by a series of [macros][macros] defined in [std::fmt][fmt] some of which
// include:
// - format!: Write formatted text to [String][string]
// - print!: Same as format! but the text is printed to the console (io::stdout)
// - println!: Same as print! but a newline is appended
// - eprint!: Same as format! but the text is printed to the standard error (io::stderr)
// - eprintln!: Same as eprint! but a newline is appended
// All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.

// Given
// fn main() {
//     let s1 = "hello";
//     /* Fill in the blank */
//     let s = format!(__);
//     assert_eq!(s, "hello, world!");
// }

// My Solution
fn main() {
    let s1 = "hello";
    let s = format!("{}, world!", s1); // Make(format) string to include s1 and equal hello, world!
    assert_eq!(s, "hello, world!");
}
