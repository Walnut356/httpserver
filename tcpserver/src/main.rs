use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:300").unwrap();
    println!("Running on port 3000");
    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        println!("Connection established");
        let mut buf = [0; 1024];

        s.read_exact(&mut buf).unwrap();
        s.write_all(&mut buf).unwrap();
    }
}
