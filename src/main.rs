mod utils;
mod errors;

use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::utils::parse_stream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp_stream) => {
                println!("accepted new connection");

            loop{
                let mut buf = [0_u8; 512];
                match tcp_stream.read(&mut buf) {
                        Ok(n) => {
                            let commands = parse_stream(&buf[..n]);

                            if let Some(request_line) = commands.get(0) {
                                let parts: Vec<&[u8]> = request_line.split(|&b| b == b' ').collect();
                                let path = parts.get(1).map(|p| *p).unwrap_or(b"/");

                                let response: Vec<u8> = match path {
                                    b"/" => b"HTTP/1.1 200 OK\r\n\r\n".to_vec(),
                                    b"/user-agent" => {
                                        let mut content = "";
                                        for line in &commands[1..]{
                                            if line.starts_with(b"User-Agent: "){
                                                content = std::str::from_utf8(&line[12..]).unwrap();
                                            }
                                        }
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content.len(), content).into_bytes()
                                    },
                                    p if p.starts_with(b"/echo/") => {
                                        let msg_str = std::str::from_utf8(&p[6..]).unwrap_or("");
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", msg_str.len(), msg_str).into_bytes()
                                    }
                                    _ => b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec(),
                                };

                                tcp_stream.write_all(&response).ok();
                            }
                        },
                        Err(e) => println!("error: {}", e),
                }
            }
        }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
