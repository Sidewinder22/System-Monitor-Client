use std::process::Command;
use crate::fs::reader::read_file;

const UPTIME_PATH : &str = "/proc/uptime";

/// get number of the clock ticks to seconds
pub fn get_clk_tck() -> u64 {

    let output = Command::new("getconf")
        .arg("CLK_TCK")
        .output()
        .expect("Failed to execute process");

    let str = String::from_utf8(output.stdout).unwrap();
    str.trim().parse::<u64>().unwrap()
}

/// get system uptime in seconds
pub fn get_system_uptime() -> u64 {
    let uptime_content = read_file(UPTIME_PATH);
    let split : Vec<&str> = uptime_content.split(" ").collect();
    split[0].trim().parse::<f64>().unwrap() as u64
}
