use std::fs;

pub fn read_file(path: &str) -> String {

    match fs::read_to_string(path) {
        Ok(content) => { return content },
        Err(e) => {
            println!("Path: {path}, Error: {e}");
            return String::from("");
        }
    }
}
