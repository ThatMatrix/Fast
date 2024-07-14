// Uncomment this block to pass the first stage
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
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
    let status_line = String::from("HTTP/1.1 200 OK\r\n\r\n");
    // TODO remove expect by handling error
    stream
        .write(status_line.as_ref())
        .expect("Couldn't write to stream");
    println!("Sent response to stream")
}
