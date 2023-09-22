use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut data: [u8; 1024] = [0; 1024];
                stream.read(&mut data).unwrap();
                let resp = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write(resp.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
