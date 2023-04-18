use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    client()
}

fn client() {
    let mut s = TcpStream::connect("localhost:8080").unwrap();
    let str = String::from("123456");
    s.write(str.as_bytes()).unwrap();

    let mut buffer = [0; 6];
    s.read(&mut buffer).unwrap();

    println!("response data {:?}", std::str::from_utf8(&buffer).unwrap())
}
