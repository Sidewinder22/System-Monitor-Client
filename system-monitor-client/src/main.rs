use network::connection::start_server;

pub mod fs;
pub mod network;

fn main() -> std::io::Result<()> {
    println!("### System Monitor Client ###");

    // let cpu = fs::cpu_average_load::get_cpu_average_load();
    // println!("cpu:{cpu}");

    start_server().expect("Server error");

    Ok(())
}
