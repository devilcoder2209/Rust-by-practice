// Given

// Fill in the blanks to make it work
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // Using the non-generic functions
//     reg_fn(__);          // Concrete type.
//     gen_spec_t(__);   // Implicitly specified type parameter `A`.
//     gen_spec_i32(__); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(__);

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(__);

//     println!("Success!");
// }

// My Solution
struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A)); // Give struct S as parameter because reg_fn takes type S which takes type A

    gen_spec_t(SGen(A)); // Give generic struct SGen as parameter and SGen with value of struct A

    gen_spec_i32(SGen(114)); // Give generic struct SGen as parameter and SGen with a random i32 integer

    generic::<char>(SGen('c')); // Give SGen with char value as parameter because we explicilty told the generic function that we are expecting a character

    generic(SGen('c')); // Give generic struct SGen as parameter and SGen should have a char value 

    println!("Success!");

}