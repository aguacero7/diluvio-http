use std::io::{self, Read, Write};
use std::net::{TcpListener,TcpStream};
use std::process::Command;
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_lines: Vec<&str> = request.lines().collect();

    if let Some(request_line) = request_lines.get(0) {
        let mut parts = request_line.split_whitespace();
        if let Some(method) = parts.next() {
            if method == "GET" || method=="POST" {
                if let Some(path) = parts.next() {
                    let mut file_path = String::from("web_documents");
                    file_path.push_str(path);

                    // Check php
                    if file_path.ends_with(".php") {
                        let output = Command::new("php")
                            .arg(&file_path)
                            .output()
                            .expect("failed to execute PHP script");

                        // proper Content-Type header for PHP script
                        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n",
                                               output.stdout.len());

                        // Write the HTTP response and PHP script output to the client
                        return stream.write_all(response.as_bytes())
                            .and_then(|_| stream.write_all(&output.stdout))
                            .map(|_| ());
                    } else {
                        let file_content = match std::fs::read(&file_path) {
                            Ok(content) => content,
                            Err(_) => {
                                return stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").map(|_| ());
                            }
                        };

                        //content type 
                        let content_type = if file_path.ends_with(".html") {
                            "text/html; charset=utf-8" // Set charset to UTF-8
                        } else if file_path.ends_with(".css") {
                            "text/css"
                        } else {
                            "text/plain"
                        };


                        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                                               content_type,
                                               file_content.len());

                        // Write the HTTP response and file content to the client
                        return stream.write_all(response.as_bytes())
                            .and_then(|_| stream.write_all(&file_content))
                            .map(|_| ());
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn start_http_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);
    for stream in listener.incoming() {
        let stream = stream.expect("failed to accept incoming connection");
        thread::spawn(move || {
            if let Err(err) = handle_client(stream) {
                eprintln!("Error handling client: {}", err);
            }
        });
    }
    Ok(())
}
