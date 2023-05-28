use std::io::prelude::*;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        spawn(move || {
            match stream {
                Ok(mut _stream) => {
                    println!("accepted new connection");

                    loop {
                        handle_connection(&mut _stream);
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            };
        });
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    let _data = stream.read(&mut buffer);

    let response = "+PONG\r\n";

    let _result = stream.write(response.as_bytes());
    stream.flush().unwrap();
}
