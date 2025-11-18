#![allow(unused)]
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Shortcut for propagating errors

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // panic!("Crash and burn");

    let v = vec![1, 2, 3, 4];

    // v[99]; - Causes a panic
    //
    // Recoverable errors with result

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // Shortcuts for panic on error with result: unwrap and except

    let greeting_file = File::open("hello.txt").unwrap(); // If result
    // is Ok -It returns the value in the Ok variant. If value is Err It calls the panic! macro

    let greeting_file = File::open("hello.txt").expect("Error while opening file");

    // Propagating errors
    //

    read_username_from_file();
    read_username_from_file_shortcut();
    read_username_from_file_shorter();
    read_username_from_file_shortest();
}
