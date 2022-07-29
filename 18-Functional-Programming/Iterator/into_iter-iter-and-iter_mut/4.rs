// In the previous section, we have mentioned that for will apply the into_iter to the collection, and
// change it into a iterator. However, this is not the only way to convert collections into iterators.
// into_iter, iter, iter_mut, all of them can convert an collection into an iterator, but in different 
// ways:
// - into_iter: Consumes the collection, once the collection has been consumed, it is no londer 
//              available for reuse, because its owenership has been moved with the loop.
// - iter: This borrows each element of the collection through each iteration, thus leaving the
//         collection untouched and available for reusue after the loop.
// - iter_mut: This mutably borrow each element of the collection, allowing for the collection to
//             be modified in place

// Given

/* Make it work */
// fn main() {
//     let arr = vec![0; 10];
//     for i in arr {
//         println!("{}", i)
//     }

//     println!("{:?}",arr);
// }

// My Solution
fn main() {
    let arr = vec![0; 10];
    for i in arr.iter() { // Make arr iteratable and use .iter() so that we can use arr later in println!
        println!("{}", i)
    }

    println!("{:?}",arr);
}