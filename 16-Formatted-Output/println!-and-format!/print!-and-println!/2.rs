// Given

// fn main() {
//     /* Fill in the blanks to make it print:
//     Hello world, I am 
//     Sunface!
//     */
//     __("hello world, ");
//     __("I am");
//     __("Sunface!");
// }
 
// My Solution
fn main() {
    /* Fill in the blanks to make it print:
    Hello world, I am 
    Sunface!
    */
    print!("hello world, "); // print! so we don't append a new line
    println!("I am"); // Use println! to print on the same line but add a new line at the end 
    println!("Sunface!");
}