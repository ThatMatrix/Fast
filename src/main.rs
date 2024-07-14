// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    path, str,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                send_response(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn send_response(mut stream: TcpStream) {
    println!("Parsing request");
    let mut buffer: Vec<u8> = Vec::new();
    // TODO if incomming connection does not close the stream, could hang forever
    // TODO remove expect in code
    stream
        .read_to_end(&mut buffer)
        .expect("Could not read request from stream");
    let request = str::from_utf8(&buffer).expect("Could not parse request in utf8");
    let mut iterator = request.split("\r\n");

    let path_requested = iterator
        .next()
        .expect("Request must have a status line")
        .split(" ")
        .nth(1)
        .expect("Status line must contain a path");
    println!("Path requested = \"{}\"", path_requested);

    println!("Sending response to stream");
    let status_line = if path_requested.len() <= 1 {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };

    // TODO remove expect by handling error
    stream
        .write(status_line.as_ref())
        .expect("Couldn't write to stream");
}
