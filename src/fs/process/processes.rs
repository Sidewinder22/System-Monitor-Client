use std::fs;
use crate::fs::process::info::{ProcessInfo, ProcessState};

pub fn read_file() {

    let path = "/proc/5195/status";

    let content = fs::read_to_string(path).expect("Can't read file!");
    let lines = content.split('\n');

    let mut process_info = ProcessInfo {
        pid: 0,
        name: String::from("Name"),
        state: ProcessState::Unknown,
        threads: 0,
    };

    println!("Before reading, name: {}", process_info.name);

    for line in lines {
        println!("line: {line} ");

        if line.contains("Name") {
            process_info.name = line.to_string();
        }
    }

    println!("After reading, name: {}", process_info.name);

}
