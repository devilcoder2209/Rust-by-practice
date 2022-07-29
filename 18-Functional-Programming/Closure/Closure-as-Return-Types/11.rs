// Given

/* Fill in the blank and fix the error*/
// fn factory(x:i32) -> __ {

//     let num = 5;

//     if x > 1{
//         move |x| x + num
//     } else {
//         move |x| x + num
//     }
// }

// My Solution
fn factory(x:i32) -> Box<dyn FnOnce(i32) -> i32> { 

    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}

fn main() {
    
}