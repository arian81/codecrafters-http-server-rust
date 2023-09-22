use std::io::prelude::*;
use std::net::TcpListener;

const BUFFER_SIZE: usize = 1024;
const SUCCESS_HTTP: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOTFOUND_HTTP: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

fn process(request: [u8; BUFFER_SIZE]) -> Vec<u8> {
    let reqstr = String::from_utf8(request.to_vec()).unwrap();
    let mut splitreq: Vec<&str> = reqstr.split("\r\n").collect();
    splitreq.truncate(splitreq.len() - 2);
    let req: Vec<&str> = splitreq[0].split_whitespace().collect();
    let _req_type = req[0];
    let path = req[1];
    let mut resp: Vec<u8> = Vec::new();

    if path.eq("/") {
        resp.extend(SUCCESS_HTTP.as_bytes());
    } else {
        resp.extend(NOTFOUND_HTTP.as_bytes());
    }
    resp
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut data: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                stream.read(&mut data).unwrap();

                stream.write(&process(data)).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
