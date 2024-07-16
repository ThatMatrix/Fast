use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};

struct Server {
    ip: String,
    port: String,
    root_directory: String,
    thread_pool: Vec<std::thread::JoinHandle<Result<(), String>>>,
}

impl Server {
    fn new(ip: String, port: String) -> Result<Server, String> {
        Ok(Server {
            ip,
            port,
            root_directory: match get_root_path() {
                Ok(root_directory) => root_directory,
                Err(err) => {
                    return Err(format!(
                        "Server::new() -> Could not infer root_directory: {}",
                        err
                    ))
                }
            },
            thread_pool: vec![],
        })
    }

    fn run(&mut self) -> Result<(), String> {
        let bind_address = format!("{}:{}", &self.ip, &self.port);
        let listener = match TcpListener::bind(bind_address.as_str()) {
            Ok(tcp_listener) => tcp_listener,
            Err(err) => {
                return Err(format!(
                    "Could not bind to target address {bind_address} - {err}",
                ))
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(_stream) => {
                    // we have to clone here because we are inside a thread and modifying root_path might
                    // include modifying it for concurrent threads
                    let _root_path = self.root_directory.clone();
                    println!("accepted new connection");
                    let handler = thread::spawn(|| send_response(_stream, _root_path));
                    self.thread_pool.push(handler);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
        Ok(())
    }

    //TODO implem clean stop function to make sure all threads are closed
}

fn main() -> Result<(), String> {
    let mut server = Server::new("127.0.0.1".to_string(), "4221".to_string())?;
    server.run()
}

fn get_root_path() -> std::io::Result<String> {
    let mut args = std::env::args();
    while args.next().is_some_and(|arg| arg != "--directory") {
        continue;
    }
    let root_path = args.next().unwrap_or_default();
    Ok(root_path)
}

fn send_response(mut stream: TcpStream, mut root_path: String) -> Result<(), String> {
    println!("Parsing request");
    let mut buffer = [0; 2048];

    // TODO if incomming connection does not close the stream, could hang forever
    // TODO remove expect in code
    let _nb_read = stream
        .read(&mut buffer)
        .expect("Could not read request from stream");
    let request = match str::from_utf8(&buffer) {
        Ok(request) => request,
        Err(err) => return Err(err.to_string()),
    };
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
    match stream.write(response.as_ref()) {
        Ok(nb_wrote) => nb_wrote,
        Err(err) => return Err(format!("Could not write file - {err}")),
    };
    Ok(())
}

fn read_file(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
