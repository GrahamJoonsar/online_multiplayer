// External Libs
use std::io::{stdin, Read, Write}; // for user input
use std::net::{TcpListener, TcpStream}; // for networking

// Internal Libs
use crate::player::Player;
use crate::net_lib::Role;
mod player;
mod net_lib;
mod input;

fn main() {
    // Getting the choice of role from the user, either server or client
    let role: Role = match input::menu_input("Select Type:".to_string(), 
    vec!["Server".to_string(), "Client".to_string()]) {
        0 => Role::Server,
        _ => Role::Client
    };
    
    match role {
        Role::Server => {
            let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to address");
            println!("Server listening on port 8080");

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => { std::thread::spawn(|| net_lib::handle_client(stream)); },
                    Err(e) => { eprintln!("Failed to establish connection: {}", e); },
                }
            }
        },

        Role::Client => {
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
                println!("Connected to the server!");
                // Send to server
                stream.write("Hello, Server!".as_bytes()).expect("Could not write to stream.");
                // Recieve response from server
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).expect("Failed to read from client");
                let response = String::from_utf8_lossy(&buffer[..]);
                println!("Received response: {}", response);
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
            } else {
                println!("Couldn't connect to server.");
            }
        }
    }
}
