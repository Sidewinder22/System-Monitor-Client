use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file: {path}!")
}
