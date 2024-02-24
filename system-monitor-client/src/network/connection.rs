use std::io::{self, Write};
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::thread::sleep;
use crate::fs::average_load::get_average_load;

const IP : &str = "127.0.0.1";
const PORT : u32 = 9999;
const DELAY: u64 = 5;

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

    loop {
        let aver_load = get_average_load();

        let result = stream
            .write(aver_load.as_bytes());
        match result {
            Ok(_) => {},
            Err(e) => {
                println!("Error during write to socket: {e}");
                break;
            }
        }

        sleep(Duration::new(DELAY, 0));
    }

    println!("Client disconnected, bye!");
    Ok(())
}
