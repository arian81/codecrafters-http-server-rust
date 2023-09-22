use std::collections::HashMap;
use std::env::{self};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{fs, thread};

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
    } else if processed_request.path.starts_with("/echo") {
        let value = processed_request.path.strip_prefix(r"/echo/").unwrap();
        resp.extend(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                value.len(),
                value
            )
            .as_bytes(),
        );
    } else if processed_request.path.eq("/user-agent") {
        let user_agent = processed_request.headers.get("User-Agent").unwrap();
        resp.extend(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            )
            .as_bytes(),
        );
    } else if processed_request.path.starts_with("/files") {
        let mut directory: String = String::new();
        let filename = processed_request.path.strip_prefix("/files/").unwrap();
        for arg in env::args() {
            if arg.starts_with("--directory") {
                directory = arg.trim_start_matches("--directory=").to_string();
                break;
            }
        }
        let filepath = Path::new(&directory).join(filename);
        dbg!(filepath.clone());
        match fs::read_to_string(filepath) {
            Ok(contents) => {
                // Construct the HTTP response
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    contents.len(),
                    contents
                );

                // Extend the response Vec<u8>
                resp.extend(response.as_bytes());
            }
            Err(_) => {
                // Handle the error case
                resp.extend(b"HTTP/1.1 404 Not Found\r\n\r\nFile not found");
            }
        }
    } else {
        resp.extend(NOTFOUND_HTTP.as_bytes());
    }
    resp
}

fn handle_request(mut stream: TcpStream) {
    let mut data: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    stream.read(&mut data).unwrap();
    stream.write(&process(data)).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // let args: Vec<String> = env::args().collect();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _thread = thread::spawn(move || handle_request(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
