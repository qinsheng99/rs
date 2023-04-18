use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[allow(dead_code)]
pub fn ser() {
    let l = TcpListener::bind("localhost:8080").unwrap();

    println!("start server 8080");

    for s in l.incoming() {
        let mut stream = s.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        stream.write(&mut buffer).unwrap();
    }
}

#[allow(dead_code)]
#[warn(unused_variables)]
fn handle_connection(_s: TcpStream) -> std::io::Result<()> {
    Ok(())
}

#[allow(dead_code)]
fn client() {
    let mut s = TcpStream::connect("localhost:8080").unwrap();
    let str = String::from("123");
    s.write(str.as_bytes()).unwrap();

    let mut buffer = [0; 3];
    s.read(&mut buffer).unwrap();

    println!("response data {:?}", std::str::from_utf8(&buffer).unwrap())
}
