use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;

use crate::fs::process::service::get_info_about_processes;

const IP : &str = "127.0.0.1";
// const IP : &str = "0.0.0.0";
const PORT : u32 = 9999;
const DELAY: u64 = 3;

pub fn start() -> std::io::Result<()> {
    let connection_settings = format!("{}:{}", IP, PORT.to_string());
    let mut stream = TcpStream::connect(connection_settings)?;
    println!("Connected to server!");

    let result = stream.write(get_hostname().as_bytes());
    match result {
        Ok(_) => {},
        Err(e) => {
            println!("Error during write to socket: {e}");
            return Ok(());
        }
    }

    loop {
        let info = get_info_about_processes();
        let result = stream.write(info.as_bytes());
        match result {
            Ok(_) => {},
            Err(e) => {
                println!("Error during write to socket: {e}");
                break;
            }
        }
        println!("{}", info);

        sleep(Duration::new(DELAY, 0));
    }

    Ok(())
}

fn get_hostname() -> String {
    let mut file = File::open("/etc/hostname").expect("Unable to open file");
    let mut hostname = String::new();
    file.read_to_string(&mut hostname)
        .expect("Unable to read file");
    hostname
}
