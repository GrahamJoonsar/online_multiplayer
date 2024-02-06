use std::net::TcpStream; // for networking
use std::io::{Read, Write}; // for user input

pub enum Role {
    Server = 0,
    Client = 1,
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from client");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello, client!".as_bytes();
    stream.write(response).expect("Failed to write response to client");
}