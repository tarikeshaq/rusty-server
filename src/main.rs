use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let html = fs::read_to_string("test.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", html);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("UNHANDLED REQUEST");
    }
}
