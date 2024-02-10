use std::io::{self, Read, Write};
use std::net::TcpStream;

const MESSAGE_SIZE: usize = 1024;

pub fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("Client connected.");

    let write_result = stream.write(b"Hello from the Rust client!");
    match write_result {
        Ok(bytes) => {
            println!("Bytes written: {}", bytes);
        }
        Err(e) => {
            eprintln!("Error during write: {}", e);
        }
    }

    let mut received_bytes = [0u8; MESSAGE_SIZE];
    let read_result = stream.read(&mut received_bytes);
    match read_result {
        Ok(bytes) => {
            println!("Bytes read: {}", bytes)
        }
        Err(e) => {
            eprintln!("Error during read: {}", e);
        }
    }

    let received_data = std::str::from_utf8(&received_bytes).expect("valid utf8");
    println!("{}", received_data);

    println!("Client disconnected, bye!");
    Ok(())
}
