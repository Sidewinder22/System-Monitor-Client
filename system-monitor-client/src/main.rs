use crate::fs::average_load;

mod fs;
mod network;

fn main() -> std::io::Result<()> {
    println!("### System Monitor Client ###");

    // fs::file_operations::read_file();

    // network::connection::start_server().expect("Server error");

    let aver_load = fs::average_load::get_average_load();
    println!("Average load: {aver_load}");


    Ok(())
}
