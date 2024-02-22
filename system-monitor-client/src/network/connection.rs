use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};

const IP : &str = "127.0.0.1";
const PORT : u32 = 9999;
const MESSAGE_SIZE: usize = 1024;

pub fn start_server() -> io::Result<()> {
    let connection_settings = format!("{}:{}", IP, PORT.to_string());

    let listener = TcpListener::bind(connection_settings)?;

    println!("Waiting for connection...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream).expect("Error in handle_connection");
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("Client connected.");

    stream
        .write(b"Hello from the Rust client!")
        .expect("Error during write to socket!");

    let mut received_bytes = [0u8; MESSAGE_SIZE];
    stream
        .read(&mut received_bytes)
        .expect("Error during read from socket!");

    let received_data = std::str::from_utf8(&received_bytes).expect("valid utf8");
    println!("{}", received_data);

    println!("Client disconnected, bye!");
    Ok(())
}
