/****************************************************/
// EXERCISE #1
/****************************************************/

// Modules let us organize the code within a crate into groups by readability and easy reuse. Module
// also controls the privacy of items, which is whether an item can be seen by outside code( public ), or
// is just an internal implementation and not available for outside code( private ).
// We have cteated a package named hello-package in the previous section, and it looks like this:
// .
// ├── Cargo.toml
// ├── src
// │   ├── lib.rs
// │   └── main.rs
// Now it's time to create some modules in the library crate and use them in the binary crate, let's start.

// Given

// Implement module front_of_house based on the module tree below:
// library crate root
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          ├── take_payment
//          └── complain

// FILL in the blank
// in __.rs

// mod front_of_house {
//     // IMPLEMENT this module..
// }

// My Solution

// in lib.rs

mod front_of_house {
    mod hosting { // Add another module for hosting
        fn add_to_waitlist() {} // Add corresponding functionality
        fn seat_at_table() {}
    }
    mod serving { // Add serving module
        fn take_order() {} // Add functionality for serving
        fn serve_order() {}
        fn take_payment() {}
        fn complain() {}
    }
}
    
/**********************************************/
// EXERCISE #2
/**********************************************/

// Let's call add_to_waitlist from a function eat_at_restaurant wich is whitin the library crate root.

// Given

// in lib.rs

// FILL in the blanks and FIX the errors
// You need make something public with `pub` to provide accessiblity for outside code `fn eat_at_restaurant()`
// mod front_of_house {
//     /* ...snip... */
// }

// pub fn eat_at_restaurant() {
//     // call add_to_waitlist with **absolute path**:
//     __.add_to_waitlist();

//     // call with **relative path** 
//      __.add_to_waitlist();
// }

// My Solution

// in lib.rs
// mod front_of_house {
//     /* ...snip... */
// } ---->>> Already defined in EXERCISE #1

pub fn eat_at_restaurant() {
    // call add_to_waitlist with **absolute path**:
    crate::front_of_house::hosting::add_to_waitlist();

    // call with **relative path** 
    front_of_house::hosting::add_to_waitlist();
}

/****************************************/
// EXERCISE #3
/****************************************/

// You can use super to import items within the parent module

// Given

// in lib.rs

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // FILL in the blank in tree ways
//         //1. using keyword `super`
//         //2. using absolute path
//         __.serve_order();
//     }

//     fn cook_order() {}
// }

// My Solution
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        
        // Using absolute path
        crate::front_of_house::serving::serve_order();

        // Using keyword super
        super::serving::serve_order();
    }

    fn cook_order() {}
}