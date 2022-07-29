// In our recently created package hello-package, add something to make the below
// code work

// Given

// fn main() {
//     assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
//      assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
// }

// My Solution
pub use crate::front_of_house::hosting; // Add in lib.rs but i have not created a crate here to keep it simple

fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
     assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
