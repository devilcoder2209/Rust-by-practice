// Given

// /* Make it work in two ways, none of them is to remove `take(movable)` away from the code
// */
// fn main() {
//     let movable = Box::new(3);

//     let consume = || {
//         println!("`movable`: {:?}", movable);
//         take(movable);
//     };

//     consume();
//     consume();
// }

// fn take<T>(_v: T) {}

// My Solution

fn main() {
    let movable = Box::new(3);

    // Way One
    // Give reference
    let consume = || {
        println!("`movable`: {:?}", movable);
        take(&movable); // Give reference to movable not ownership
    };

    // Way Two
    // Give Clone
    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable.clone());
    };

    consume();
    consume();
}

fn take<T>(_v: T) {}
