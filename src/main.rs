use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;

struct RedisRequest {
    args_len: usize,
    command: String,
    message: String,
    data: Vec<String>,
}

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

    let rr = read_redis_request(&mut reader);

    let response = "+PONG\r\n";

    if validate_echo_command(&rr) {
        let response = &handle_echo_data(&rr);
        if response.len() != rr.args_len {
            let _result = writer
                .write(new_error_message("ERR wrong number of arguments for command").as_bytes());
        } else {
            let _result = writer.write(response.as_bytes());
        }
    } else {
        let _result = writer.write(response.as_bytes());
    }

    writer.flush().unwrap();
}

fn read_redis_request(reader: &mut BufReader<&TcpStream>) -> RedisRequest {
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

    let data: Vec<String> = message.split("\\r\\n").map(str::to_string).collect();

    let args_len = data.get(1).unwrap_or(&String::new()).parse().unwrap();
    let command = data.get(2).unwrap_or(&String::new()).to_string();

    let rr = RedisRequest {
        args_len,
        command,
        message,
        data,
    };

    rr
}

fn validate_echo_command(rr: &RedisRequest) -> bool {
    rr.command == "ECHO"
}

fn handle_echo_data(rr: &RedisRequest) -> String {
    let message = rr.data.get(3..).unwrap().join(" ");

    message
}

fn new_error_message(message: &str) -> String {
    String::from("-Error ") + message
}
