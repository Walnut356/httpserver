use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    stream.write_all(b"Hello").unwrap();

    let mut buf = [0; 5];
    stream.read_exact(&mut buf).unwrap();

    println!("Got response from server: {:?}", str::from_utf8(&buf).unwrap())
}
