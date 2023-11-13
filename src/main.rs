use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::thread;

const CURRENT_IP: &str = "192.168.1.35:7878";

fn main() {
    let is_client = true;

    if is_client {
        launch_client();
    } else {
        launch_server();
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 { break; }

        stream.write(&buffer[..bytes_read]).unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
}

fn launch_server() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn launch_client() {
    let mut stream = TcpStream::connect(CURRENT_IP).expect("Could not connect to server");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        stream.write(input.as_bytes()).expect("Failed to write to stream");

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
}