use network::connection::start_server;

pub mod fs;
pub mod network;
pub mod tools;

fn main() -> std::io::Result<()> {

    println!("### System Monitor Client ###");

    start_server().expect("Server error");

    Ok(())
}
