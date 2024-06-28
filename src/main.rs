use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    //thread,
    //time::Duration,
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
    let status_line = "HTTP/1.1 200 OK";

    let mut request = |response: &str, mut status: &str| {
        let contents = fs::read_to_string(response).unwrap();
        let length = contents.len();

        if status.is_empty() {
            status = status_line;
        }

        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status, length, contents);

        stream.write_all(response.as_bytes()).unwrap();
    };

    //println!("{:?}", request_line);
    
    match &request_line[..] {
        // Main page
        "GET / HTTP/1.1" => request("./static/index.html", ""),
        // Download page
        "GET /download HTTP/1.1" => request("./static/download.html", ""),
        // script js
        "GET /pkg/rok_page.js HTTP/1.1" => request("./pkg/rok_page.js", ""),
        // script wasm
        "GET /pkg/rok_page_bg.wasm HTTP/1.1" => request("./pkg/rok_page_bg.wasm", ""),
        // not found
        _ => request("./static/404.html", "HTTP/1.1 404 NOT FOUND"),
    }
}