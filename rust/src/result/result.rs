use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{fs, io};
#[allow(dead_code)]
pub fn open1() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("creat file err {:?}", e)
                }
            },
            _other => {
                panic!("open file err {:?}", error)
            }
        },
    };
}

#[allow(dead_code)]
pub fn open2() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| panic!("creat file err {:?}", error))
        } else {
            panic!("open file err {:?}", error)
        }
    });

    let _ff = File::open("").unwrap();
}

#[allow(dead_code)]
fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(f)
}

#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
