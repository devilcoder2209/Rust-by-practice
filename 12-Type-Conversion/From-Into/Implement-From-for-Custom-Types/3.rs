// When performing error handling it is often useful to implement the From trait for our own
// error type. Then we can use ? to automatically convert the underlying error type to our own
// error type

// Given

// use std::fs;
// use std::io;
// use std::num;

// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }

// impl From<io::Error> for CliError {
//     // IMPLEMENT from method
// }

// impl From<num::ParseIntError> for CliError {
//     // IMPLEMENT from method
// }

// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? automatically converts io::Error to CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }

// fn main() {
//     println!("Success!")
// }

// My Solution
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err) // Return IoError variant of enum with the error
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> Self {
        CliError::ParseError(err) // Return ParseIntError variant of enum with the error
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!")
}