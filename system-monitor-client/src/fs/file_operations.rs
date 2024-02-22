use std::fs;

pub fn read_file() {

    let path = "/proc/1681/status";

    let content = fs::read_to_string(path).expect("Can't read file!");

    println!("Text:\n{content}");
}
