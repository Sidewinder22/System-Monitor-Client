use crate::fs::reader::read_file;

const AVERAGE_LOAD_PATH : &str = "/proc/loadavg";

pub fn get_average_load() -> String {
    let mut aver_load = read_file(AVERAGE_LOAD_PATH);
    
    aver_load.pop(); // remove \n char
    
    format!("AVERAGE_LOAD: {} END", aver_load)
}
