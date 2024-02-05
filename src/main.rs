use std::io::{stdin, Read, Write}; // for user input
use std::net::{TcpListener, TcpStream}; // for networking

// Generates a text menu for the user to select between choices by
// inputting a number from 0 to choices.len()-1
fn menu_input(prompt: String, choices: Vec<String>) -> usize {
    // Display the choices to the user
    println!("{}", prompt);
    for i in 0..choices.len() {
        println!("{}: {}", i, choices[i]);
    }

    loop {
        // Gather input repeatedly, til the user enters a valid number
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(n) => {
                if 0 <= n && n < choices.len() as i32 {
                    return n as usize;
                } else {
                    println!("\nMake sure your input is one of the choices.");
                }
            },
            Err(e) => {
                println!("\nMake sure you are entering a valid number.\nError: {}", e);
            }
        }
    }
}

enum Role {
    Server = 0,
    Client = 1,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from client");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello, client!".as_bytes();
    stream.write(response).expect("Failed to write response to client");
}

fn main() {
    // Getting the choice of role from the user, either server or client
    let role: Role = match menu_input("Select Type:".to_string(), 
    vec!["Server".to_string(), "Client".to_string()]) {
        0 => Role::Server,
        _ => Role::Client
    };
    
    match role {
        Role::Server => {
            let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
            println!("Server listening on 127.0.0.1:8080");

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => { std::thread::spawn(|| handle_client(stream)); },
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
