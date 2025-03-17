use network::tcp_client;

pub mod fs;
pub mod network;
pub mod tools;

fn main() -> std::io::Result<()> {

    println!("### System Monitor Client ###");
    tcp_client::start().expect("Client error");
    Ok(())
}
