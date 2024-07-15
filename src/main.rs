use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn main() {
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
    // let mut buffer: Vec<u8> = Vec::new();
    let mut buffer = [0; 2048];

    // TODO if incomming connection does not close the stream, could hang forever
    // TODO remove expect in code
    let _nb_read = stream
        .read(&mut buffer)
        .expect("Could not read request from stream");
    let request = str::from_utf8(&buffer).expect("Could not parse request in utf8");
    let mut iterator = request.split("\r\n");

    // TODO remove expect
    let path_requested = iterator
        .next()
        .expect("Request must have a status line")
        .split(" ")
        .nth(1)
        .expect("Status line must contain a path");

    // TODO check if path requested contains echo
    let response = if path_requested.len() <= 1 {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else if path_requested.starts_with("/echo/") {
        // TODO if contains echo then get the rest of the path
        let body = path_requested.trim_start_matches("/echo/");
        let body_length = body.len();
        // TODO send back the rest of the path inside the body with Content-type header
        String::from(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:{body_length}\r\n\r\n{body}"))
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };

    println!("Sending response to stream for {}", path_requested);

    // TODO remove expect by handling error
    stream
        .write(response.as_ref())
        .expect("Couldn't write to stream");
}
