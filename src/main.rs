use domain::resp::Resp;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;

mod domain;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        spawn(move || {
            match stream {
                Ok(_stream) => {
                    println!("accepted new connection");

                    loop {
                        handle_connection(&_stream);
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            };
        });
    }
}

fn handle_connection(stream: &TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);

    let resp = read_redis_request(&mut reader);

    let response = resp.response();

    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn read_redis_request(reader: &mut BufReader<&TcpStream>) -> Resp {
    let mut message = String::new();

    let _data = match reader.read_line(&mut message) {
        Err(e) => match e.kind() {
            io::ErrorKind::WouldBlock => {
                println!("would have blocked");
            }
            _ => {
                println!("Got an error: {}", e)
            }
        },
        Ok(m) => {
            if m > 0 {
                println!("Received {:?}, {:?}", m, message);
            }
        }
    };

    Resp::new(message)
}
