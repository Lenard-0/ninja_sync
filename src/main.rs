use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::{thread, env};

fn main() {
    dotenv::dotenv().ok();
    let is_client = env::var("IS_CLIENT").unwrap_or("false".to_string()) == "true";

    if is_client {
        launch_client();
    } else {
        launch_server();
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    let read_thread = thread::spawn(move || {
        read_from_stream(stream);
    });

    write_to_stream(stream_clone);

    read_thread.join().unwrap();
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
    let mut stream = match TcpStream::connect(env::var("IP_ADDRESS").unwrap()) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to server: {:?}", e);
            return;
        }
    };

    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    let read_thread = thread::spawn(move || {
        read_from_stream(stream);
    });

    write_to_stream(stream_clone);

    read_thread.join().unwrap();
}

fn read_from_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 { break; }
                println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {:?}", e);
                break;
            }
        }
    }
}

fn write_to_stream(mut stream: TcpStream) {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            break;
        }

        if stream.write(input.as_bytes()).is_err() {
            eprintln!("Failed to write to stream");
            break;
        }
    }
}