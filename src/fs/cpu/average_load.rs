use crate::fs::reader::read_file;

const AVERAGE_LOAD_PATH : &str = "/proc/loadavg";

pub fn get_average_load() -> String {
    let aver_load = read_file(AVERAGE_LOAD_PATH);
    format!("AVERAGE_LOAD: {} \n", aver_load)
}
