use std::io::{self, Read, Write};
use std::net::TcpStream;

const MESSAGE_SIZE: usize = 1024;

pub fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
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
