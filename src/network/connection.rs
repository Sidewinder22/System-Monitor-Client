use std::io::{self, Write};
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::thread::sleep;

use crate::fs::cpu::average_load::get_average_load;
use crate::fs::cpu::cpu_load_provider::CpuLoadProvider;
use crate::fs::process::paths::get_paths;

use crate::fs::process::cpu_load::ProcessCpuLoad;

const IP : &str = "127.0.0.1";
const PORT : u32 = 9999;
// const DELAY: u64 = 5;
const DELAY: u64 = 1;

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

    let mut cpu_average_load = CpuLoadProvider::new();
    let mut process_cpu_load = ProcessCpuLoad::new();

    loop {
        let aver_load = get_average_load();
        let cpu_aver_load = cpu_average_load.get_cpu_average_load();

        let process_cpu_load = process_cpu_load.get_process_load(1198);

        let mut message : String = Default::default();
        message.push_str("Process load: ");
        message.push_str(&process_cpu_load.to_string());
        message.push_str("\n");
        message.push_str(&aver_load);
        message.push_str("\n");
        message.push_str(&cpu_aver_load);

        let paths = get_paths().expect("Problem with paths preparing");
        let mut counter = 0;

        for path in paths {
            if counter == 10 {
                break;
            }

            message.push_str("\n");
            message.push_str(path.to_str().expect("Problem with path!"));
            counter = counter + 1;
        }

        let result = stream
            .write(message.as_bytes());
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
