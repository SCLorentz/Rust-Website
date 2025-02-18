use std::alloc::System;

#[global_allocator]
static A: System = System;

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
    //
    let (request_line, status_line) = (
        // request_line
        BufReader::new(&mut stream).lines().next().unwrap().unwrap(), 
        // status_line
        "HTTP/1.1 200 OK"
    );

    let mut request = |response: &str, mut status: &str, r#type: &str| {
        // prevent panic
        let contents = match fs::read_to_string(response) {
            Ok(contents) => contents,
            Err(_) => {
                return {
                    let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}",
                        fs::read_to_string("./static/err/404.html").unwrap()
                    );
                    //
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        };

        if status.is_empty() {
            status = status_line;
        }

        //let response = format!("{}\r\n\r\n{}", status, contents);

        //stream.write_all(response.as_bytes()).unwrap();
        let content_type = match r#type {
            "wasm" => "application/wasm", // Set Content-Type for WASM
            "js" => "text/javascript",
            _ => "text/html", // Default content type
        };
    
        let response = format!("{}\r\nContent-Type: {}\r\n\r\n{}", status, content_type, contents);
    
        stream.write_all(response.as_bytes()).unwrap();
    };
    
    match &request_line[..] {
        // Main page
        "GET / HTTP/1.1" => request("./static/index.html", "", ""),
        // Download page
        "GET /download HTTP/1.1" => request("./static/pages/download.html", "", ""),
        // script js
        "GET /script HTTP/1.1" => request("./static/awesome.js", "", "js"),
        // script wasm
        "GET /wasm HTTP/1.1" => request("./static/awesome_bg.wasm", "", "wasm"),
        // not found
        _ => request("./static/404.html", "HTTP/1.1 404 NOT FOUND", ""),
    }
}

// cara, o nome do commit ficou mt bom "vey",  se fude essa porra, to rindo muito, mas isso não faz com que não seja verdade, infelismente