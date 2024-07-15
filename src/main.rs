use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};

use nom::combinator::iterator;

fn main() {
    let root_path = get_root_path();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // we have to clone here because we are inside a thread and modifying root_path might
                // include modifying it for concurrent threads
                let _root_path = root_path.clone();
                println!("accepted new connection");
                thread::spawn(|| send_response(_stream, _root_path));
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
    let root_path = args.next().unwrap_or_default();
    root_path
}

fn send_response(mut stream: TcpStream, mut root_path: String) {
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
    let mut status_line = iterator
        .next()
        .expect("Request must have a status line")
        .split(" ");
    let method = status_line
        .next()
        .expect("Status line must contain a method");
    let path_requested = status_line.next().expect("Status line must contain a path");

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
        let user_agent = iterator
            .find(|header| header.starts_with("User-Agent: "))
            .unwrap_or("")
            .trim_start_matches("User-Agent: ");
        let user_agent_len = user_agent.len();

        String::from(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:{user_agent_len}\r\n\r\n{user_agent}"))
    } else if path_requested.starts_with("/files/") {
        let path = path_requested.trim_start_matches("/files/");
        root_path = root_path + path;

        if method == "GET" {
            match read_file(root_path) {
                Ok(body) => {
                    let body_length = body.len();
                    String::from(format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length:{body_length}\r\n\r\n{body}"
                ))
                }
                Err(_) => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
            }
        }
        // Method is POST
        else {
            while iterator.next().is_some_and(|str| str.len() > 0) {
                continue;
            }
            let body_request = iterator
                .next()
                .unwrap()
                .trim_end_matches(|c| c == '\0')
                .as_ref();

            let mut file = File::create(root_path).expect("Could not create file");
            file.write_all(body_request)
                .expect("Could not write to file");
            String::from("HTTP/1.1 201 Created\r\n\r\n")
        }
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };

    println!("Sending response to stream for {}", path_requested);

    // TODO remove expect by handling error
    stream
        .write(response.as_ref())
        .expect("Couldn't write to stream");
}

fn read_file(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
