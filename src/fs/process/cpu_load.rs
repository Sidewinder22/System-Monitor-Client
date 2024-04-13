use crate::fs::reader::read_file;
use crate::fs::process::stat_parser;
use crate::tools::system::{get_clk_tck, get_system_uptime};

pub fn calculate(pid : u32) -> f64 {

    let path = format!("/proc/{}/stat", pid);
    let stat = stat_parser::parse(&read_file(&path));

    let clk_tck = get_clk_tck();
    let uptime_sec = get_system_uptime();

    let utime_sec = stat.utime / clk_tck;
    let stime_sec = stat.stime / clk_tck;
    let starttime_sec = stat.starttime / clk_tck;
    let elapsed_sec = uptime_sec - starttime_sec;
    let usage_sec = utime_sec + stime_sec;

    (usage_sec * 100) as f64 / (elapsed_sec as f64)
}
