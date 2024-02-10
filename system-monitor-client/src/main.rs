use std::net::TcpListener;

use system_monitor_client::handle_connection;

fn main() -> std::io::Result<()> {

    println!("### System Monitor Client ###");

    let ip = "127.0.0.1";
    let port = 9999;
    let connection_settings = format!("{}:{}", ip, port.to_string());

    let listener = TcpListener::bind(connection_settings)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let result = handle_connection(stream);
                match result {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error in handle_connection: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
