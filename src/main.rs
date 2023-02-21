extern crate core;

mod home_controller;
mod request_simple_parser;
mod request_handler;
mod request;
mod response;
mod controller;
mod controller_404;

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use regex::{Error, Regex};
use crate::request_handler::handle_request;
use crate::request_simple_parser::parse_request;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream).expect("TODO: panic message");
        println!("Connection established!")
    }
}

fn handle_connection(mut stream: TcpStream) ->  Result<String, io::Error>  {
    let buf_reader = BufReader::new(&mut stream);

    let http_request = match buf_reader.
        lines().next() {
        None => String::from("GET / HTTP/1.1"),
        Some(x) => match x {
            Ok(r) => r,
            Err(err) => return Err(err)
        }
    };
    let response = handle_request(parse_request(http_request).unwrap()).unwrap();

    let length = response.get_length();
    let content = response.get_body();
    let status_line = response.get_status_code_body();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();

    //println!("type='{:?}', location='{}'", request.type_request, request.location_raw);
    Ok(String::from("request handled"))
}
