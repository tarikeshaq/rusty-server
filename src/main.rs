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

    let (status_line, filepath) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n{}", "test.html")
        
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "notfound.html")
    };
    let response = format!("{}{}", status_line, fs::read_to_string(filepath).unwrap());
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
