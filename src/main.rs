use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;

struct RedisRequest {
    n_data: usize,
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

    let mut response = String::from("+PONG\r\n");

    if validate_echo_command(&rr) {
        response = handle_echo_data(&rr);
        if (rr.data.len() + 1) != rr.n_data {
            response = new_error_message("ERR wrong number of arguments for command");
            println!(
                "ERR wrong number of arguments for command: {}, {}",
                rr.data.len(),
                rr.n_data
            );
        }
    }

    writer.write_all(response.as_bytes()).unwrap();
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

    parse_message(message)
}

fn parse_message(message: String) -> RedisRequest {
    let commands: Vec<String> = message.split("\\r\\n").map(str::to_string).collect();

    let n_data: usize = commands
        .get(0)
        .unwrap_or(&String::new())
        .replace("*", "")
        .parse()
        .unwrap_or(0);

    let mut contents: Vec<String> = vec![String::new(); n_data];
    for i in 0..n_data {
        contents[i] = commands.get((i + 1) * 2).unwrap().to_string();
    }
    let command = contents.get(0).unwrap_or(&String::new()).to_string();
    let data = contents.get(1..).unwrap_or_default().to_vec();

    let rr = RedisRequest {
        n_data,
        message,
        command,
        data,
    };

    rr
}

fn validate_echo_command(rr: &RedisRequest) -> bool {
    rr.command == "ECHO"
}

fn handle_echo_data(rr: &RedisRequest) -> String {
    let message = rr.data.join(" ");

    message
}

fn new_error_message(message: &str) -> String {
    String::from("-Error ") + message
}
