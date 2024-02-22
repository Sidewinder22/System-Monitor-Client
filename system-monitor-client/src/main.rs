// use std::net::TcpListener;

// use system_monitor_client::handle_connection;

mod fs;

fn main() -> std::io::Result<()> {
    println!("### System Monitor Client ###");

    // fs::read_file();
    // file_operations::read_file();
    fs::file_operations::read_file();

    // const IP : &str = "127.0.0.1";
    // const PORT : u32 = 9999;
    
    // let connection_settings = format!("{}:{}", IP, PORT.to_string());

    // let listener = TcpListener::bind(connection_settings)?;

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             handle_connection(stream).expect("Error in handle_connection");
    //         }
    //         Err(e) => {
    //             println!("Connection failed: {}", e);
    //         }
    //     }
    // }
    Ok(())
}
