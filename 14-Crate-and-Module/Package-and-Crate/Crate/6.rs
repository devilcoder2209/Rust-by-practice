// A package can contain at most one library crate, but it can contain as many binary crates
// as you would like by placing files in src/bin directory: each file will be a seperate binary
// crate with the same name as the file.

// Given

// # Create a package which contains 
// # 1. three binary crates: `hello-package`, `main1` and `main2`
// # 2. one library crate
// # describe the directory tree below
// .
// ├── Cargo.toml
// ├── Cargo.lock
// ├── src
// │   ├── __
// │   ├── __
// │   └── __
// │       └── __
// │       └── __
// ├── tests # directory for integrated tests files
// │   └── some_integration_tests.rs
// ├── benches # dir for benchmark files
// │   └── simple_bench.rs
// └── examples # dir for example files
//     └── simple_example.rs

// My Solution

// ├── Cargo.toml
// ├── Cargo.lock
// ├── src
// │   ├── main.rs
// │   ├── lib.rs
// │   └── bin
// │       └── main1.rs
// │       └── main2.rs
// ├── tests # directory for integrated tests files
// │   └── some_integration_tests.rs
// ├── benches # dir for benchmark files
// │   └── simple_bench.rs
// └── examples # dir for example files
//     └── simple_example.rs