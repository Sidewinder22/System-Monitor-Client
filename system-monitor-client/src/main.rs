mod fs;
mod network;

fn main() -> std::io::Result<()> {
    println!("### System Monitor Client ###");

    fs::file_operations::read_file();

    network::connection::start_server().expect("Server error");

    Ok(())
}
