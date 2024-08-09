/*
enum Result<T, E> {
    Ok(T), - returns T if okay
    Err(E), - returns Error if error
} */
// panic!("too bad!") // unrecoverable errors (cleans up all used memory)
use std::error;
use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    // Box<dyn Error> - means any error
    // won't raise error,  but will return it
    let file_name = "sample.txt";
    let result = File::open(file_name);

    // you can use both 'if let' and 'match' to handle results
    if let Err(error) = result {
        if let ErrorKind::NotFound = error.kind() {
            println!("File doesn't exist, creating one...");
            let file_create = File::create(file_name);
            match file_create {
                Ok(file) => println!("File created: {:?}", file),
                Err(error) => panic!("Couldn't create a file: {}", error),
            }
        } else {
            panic!("Error happened for unkown reason: {}", error);
        }
    } else if let Ok(file) = result {
        println!("file's content: {:?}", file);
    }

    // but no need to use 'if let' or 'match' - too much lines. use .unwrap()
    let fn2 = "emil.txt";
    // will extract value or will panic;
    let _file = File::open(file_name).unwrap();
    // .expect() - same as .unwrap(), but better, bc you can also specify panic message:
    let mut file2 = File::open(fn2).expect("LOL!!!!");

    // you need to write to buffer to read a content:
    let mut file2_content = String::new();
    file2
        .read_to_string(&mut file2_content)
        .expect("Couldn't read to string");
    println!("{}", file2_content);
    Ok(())
}

fn _read_file() -> Result<String, Error> {
    // signifies that it will return Ok(string) or Error
    let mut content = String::new();

    let result = File::open("hello.txt");
    let mut our_file = match result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    match our_file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

// there is shortcut to automatically return Error/value without using match
fn _read_file_shorter() -> Result<String, Error> {
    let mut content = String::new();

    let mut our_file = File::open("hello.txt")?; // gives value or terminates with Err
                                                 // ? operator will convert error to same type as specified in function result
    our_file.read_to_string(&mut content)?;
    // even shorter: File::open("hello.txt")?.read_to_string(&mut content)?;
    Ok(content)
}

// even more shorter than above:
use std::fs;
fn _read_file_megashort() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

// ? operator works for Result<>(returns an error) and Option<> (returns a None)
fn _get_last_num() -> Option<i32> {
    let nums = [32, 2, 11];
    let last = nums.get(3)?; // returns None
    Some(last * 2)
}
