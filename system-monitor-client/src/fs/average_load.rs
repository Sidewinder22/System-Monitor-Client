use crate::fs::reader::read_file;

pub fn get_average_load() -> String {
    let aver_load_path = String::from("/proc/loadavg");

    let mut aver_load = read_file(&aver_load_path);
    aver_load.pop();    // remove \n 
    
    let mut message = String::from("Average load: ");
    message.push_str(&aver_load);
    message.push_str(" END_MESSAGE");

    message
}
