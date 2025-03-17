use std::collections::HashMap;
use std::path::PathBuf;

use crate::fs::reader::read_file;

use super::process_load_calculator::ProcessLoadCalculator;
use super::paths::get_paths;
use super::status::ProcessStatus;
use super::status_parser;

const LOAD_TRESHOLD : f64 = 0.5;

pub fn get_info_about_processes() -> String {
    
    let mut message : String = Default::default();
    let process_data = get_processes();

    for data in process_data {
        let cpu_load = data.1.to_string();

        let process_status = get_process_status(data.0.display().to_string());

        message.push_str("[");
        message.push_str(&process_status.pid.to_string());
        message.push_str(" / ");
        message.push_str(&process_status.state.to_string());
        message.push_str("]: ");
        message.push_str(&process_status.name);
        message.push_str(", threads: ");
        message.push_str(&process_status.threads.to_string());
        message.push_str(", cpu_load: ");
        message.push_str(&cpu_load);
        message.push_str("\n");
    }

    message
}

fn get_processes() -> HashMap<PathBuf, f64> {

    let paths = get_paths().expect("Cannot get paths");
    let mut process_path_to_load = HashMap::new();

    for path in paths {
        let cpu_load = ProcessLoadCalculator::new().get_process_load(&path);
        process_path_to_load.insert(path, cpu_load);
    }

    let mut most_active_processes = HashMap::new();

    for process in process_path_to_load {
        if process.1 > LOAD_TRESHOLD {
            most_active_processes.insert(process.0, process.1);
        }
    }

    most_active_processes
}

fn get_process_status(path : String) -> ProcessStatus {
    let full_path = path + "/status";
    let content = read_file(&full_path);

    let lines = content.split("\n").map(|l| l.to_string()).collect();
    status_parser::parse(&lines)
}

