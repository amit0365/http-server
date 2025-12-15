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

                let mut buf = [0_u8; 512];
                match tcp_stream.read(&mut buf) {
                        Ok(n) => {
                            let commands = parse_stream(&buf[..n]);
                            if commands.get(0).map(|c| &**c) == Some(b"GET".as_slice()) {
                                let response = b"HTTP/1.1 200 OK\r\n\r\n";
                                tcp_stream.write_all(response).ok();
                            }
                        },
                        Err(e) => println!("error: {}", e),
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
