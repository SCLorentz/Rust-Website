use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut status_line = "HTTP/1.1 200 OK";

    let mut request = |response: &str| {
        let contents = fs::read_to_string(response).unwrap();
        let length = contents.len();

        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);

        stream.write_all(response.as_bytes()).unwrap();
    };

    //println!("{:?}", request_line);
    
    match request_line {
        c if c == "GET / HTTP/1.1" => request("./static/index.html"),
        c if c == "GET /download HTTP/1.1" => request("./static/download.html"),
        c if c == "GET /pkg/rok_page.js HTTP/1.1" => request("./pkg/rok_page_bg.js"),
        c if c == "GET /pkg/rok_page_bg.wasm HTTP/1.1" => request("./pkg/rok_page_bg.wasm"),
        _ => {
            status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("./static/404.html").unwrap();
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}