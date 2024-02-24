use crate::fs::reader::read_file;

pub fn get_average_load() -> String {
    let aver_load_path = String::from("/proc/loadavg");
    read_file(&aver_load_path)
}
