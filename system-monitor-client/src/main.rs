use network::connection::start_server;

pub mod fs;
pub mod network;

fn main() -> std::io::Result<()> {

    println!("### System Monitor Client ###");

    start_server().expect("Server error");

    Ok(())
}
