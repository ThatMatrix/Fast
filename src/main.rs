use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
    str, thread,
};

fn main() {
    let root_path = get_root_path();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                thread::spawn(|| send_response(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn get_root_path() -> String {
    let mut args = std::env::args();
    while args.next().is_some_and(|arg| arg != "--directory") {
        continue;
    }
    // TODO remove expect here and return nicely
    let root_path = args.next().expect("Root Path not given as parameter");
    root_path
}

fn send_response(mut stream: TcpStream) {
    println!("Parsing request");
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
    } else if path_requested == "/user-agent" {
        // TODO parse headers to find user agent
        // TODO send user agent in body (same shit as echo)
        let user_agent = iterator
            .find(|header| header.starts_with("User-Agent: "))
            .unwrap_or("")
            .trim_start_matches("User-Agent: ");
        let user_agent_len = user_agent.len();

        String::from(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:{user_agent_len}\r\n\r\n{user_agent}"))
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };

    println!("Sending response to stream for {}", path_requested);

    // TODO remove expect by handling error
    stream
        .write(response.as_ref())
        .expect("Couldn't write to stream");
}
