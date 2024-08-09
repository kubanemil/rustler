/*
TCP (Transmission Control Protocol) - defines how data is transfered (but not what is a data),
HTTP is build on top of TCP, and defines content of transfered data. */
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, thread, time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listener is created");

    // A stream is a connection between a client and the server.
    // Will print multiple connection, even if you made one, because browser
    // makes multiple retries to connect.
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("\nConnection is established!");
        
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {}", http_request[0]);

    let (status_line, filename) = match http_request[0].as_str() {
        "GET / HTTP/1.1"  => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 Not FounD", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap(); // html text
    let len = content.len();
    /* Response should be in format of:
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body
    */
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
