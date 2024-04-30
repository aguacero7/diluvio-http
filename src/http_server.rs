use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;
use std::fs;
use std::path::Path;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_lines: Vec<&str> = request.lines().collect();

    if let Some(request_line) = request_lines.get(0) {
        let mut parts = request_line.split_whitespace();
        if let Some(method) = parts.next() {
            if method == "GET" || method == "POST" {
                if let Some(path) = parts.next() {
                    let file_path = format!("web_documents{}", path);
                    let file_path = if file_path.ends_with("/") {
                        format!("{}/index.html", file_path)
                    } else {
                        file_path
                    };

                    if Path::new(&file_path).exists() {
                        if file_path.ends_with(".php") {
                            let output = Command::new("php")
                                .arg(&file_path)
                                .output()
                                .expect("failed to execute PHP script");
                        
                            let php_response = String::from_utf8_lossy(&output.stdout);
                            
                            stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
                            stream.write_all(b"Content-Type: text/html; charset=UTF-8\r\n")?;
                            stream.write_all(b"\r\n")?;
                            stream.write_all(php_response.as_bytes())?;
                        } else {
                            let file_content = fs::read(&file_path)?;

                            let content_type = if file_path.ends_with(".html") {
                                "text/html; charset=utf-8" // Set charset to UTF-8
                            } else if file_path.ends_with(".css") {
                                "text/css"
                            } else {
                                "text/plain"
                            };

                            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                                                   content_type,
                                                   file_content.len(),
                                                   String::from_utf8_lossy(&file_content));

                            stream.write_all(response.as_bytes())?;
                        }
                    }  else {
                        let error_page = r#"
                            <!DOCTYPE html>
                            <html lang="en">
                            <head>
                                <meta charset="UTF-8">
                                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                                <title>Error Page</title>
                                <style>
                                    body {
                                        font-family: Arial, sans-serif;
                                        background-color: #f4f4f4;
                                        margin: 0;
                                        padding: 0;
                                    }

                                    .container {
                                        display: flex;
                                        justify-content: center;
                                        align-items: center;
                                        height: 100vh;
                                    }

                                    .card {
                                        background-color: #ffffff;
                                        padding: 20px;
                                        border-radius: 10px;
                                        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                                    }

                                    h1 {
                                        text-align:center;
                                        font-size: 35px;
                                        font-weight: bold;
                                        color: #333333;
                                        margin-bottom: 20px;
                                    }

                                    p {
                                        color: #666666;
                                        margin-bottom: 20px;
                                    }

                                    .error-message {
                                        color: #ff0000;
                                        font-weight: bold;
                                        margin-bottom: 20px;
                                    }
                                </style>
                            </head>
                            <body>
                                <div class="container">
                                    <div class="card">
                                        <h1>404 Not Found</h1>
                                        <p class="error-message">Sorry, the page you are looking for could not be found.</p>
                                        <p>Please try again later or contact support for assistance.</p>
                                    </div>
                                </div>
                            </body>
                            </html>
                            "#;

                        let response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                                                error_page.len(),
                                                error_page);

                        stream.write_all(response.as_bytes())?;
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
