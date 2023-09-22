use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;

const BUFFER_SIZE: usize = 1024;
const SUCCESS_HTTP: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOTFOUND_HTTP: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

#[derive(Debug, Default)]
struct HTTTPRequest<'a> {
    method: &'a str,
    path: &'a str,
    headers: HashMap<&'a str, &'a str>,
    body: String,
}

fn process(request: [u8; BUFFER_SIZE]) -> Vec<u8> {
    let mut processed_request = HTTTPRequest::default();
    let rawreq_str: String = String::from_utf8(request.to_vec()).unwrap();
    let rawreq_arr: Vec<&str> = rawreq_str.split("\r\n").collect();

    // .unwrap()
    // .split("\r\n")
    // .collect();
    for i in rawreq_arr {
        if i.contains("GET") || i.contains("POST") {
            let req: Vec<&str> = i.split_whitespace().collect();
            processed_request.method = req[0];
            processed_request.path = req[1];
        } else if i.contains(": ") {
            let header: Vec<&str> = i.split(": ").collect();
            processed_request.headers.insert(header[0], header[1]);
        } else if !i.is_empty() {
            let body = i.trim_matches('\0');
            processed_request.body.push_str(body);
            processed_request.body.push_str("\n");
        }
    }

    let mut resp: Vec<u8> = Vec::new();

    if processed_request.path.eq("/") {
        resp.extend(SUCCESS_HTTP.as_bytes());
    } else if processed_request.path.contains("echo") {
        resp.extend(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                processed_request.body.len(),
                processed_request.body
            )
            .as_bytes(),
        );
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
