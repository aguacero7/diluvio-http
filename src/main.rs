use std::net::{TcpListener, TcpStream}; 
use std::io::{Read, Write}; 
use std::thread; 

fn handle_client(mut stream: TcpStream) { 
    let mut buffer = [0; 1024]; // Declare a mutable buffer of size 1024 to store incoming data
    stream.read(&mut buffer).unwrap(); // Read data from the stream into the buffer

    // Process the request here
    let request = String::from_utf8_lossy(&buffer[..]); // Convert buffer to UTF-8 string, handling invalid UTF-8 sequences
    println!("Received request: {}", request); 
    let ip = stream.peer_addr().unwrap(); // Get the IP address of the client
    // Process the request here
    println!("Connection established! {} is connected", ip);
    // Send response
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello,"; 
    stream.write(response.as_bytes()).unwrap(); 
    stream.flush().unwrap(); 
}

fn main() { 
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap(); 
    println!("Listening on port 3000..."); 

    for stream in listener.incoming() { 
        match stream { 
            Ok(stream) => { 
                thread::spawn(|| { 
                    handle_client(stream); 
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}