use std::{fs, io};
use std::path::{Path, PathBuf};
use regex::Regex;
use lazy_static::lazy_static;

const DIR_REGEX: &str = r"[0-9]";
const STAT: &str = "stat";

lazy_static! {
    static ref RE: Regex = Regex::new(DIR_REGEX).unwrap();
}

pub fn get_paths() -> io::Result<Vec<PathBuf>> {

    let mut entries = fs::read_dir("/proc")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let mut paths : Vec<PathBuf> = Vec::new();
    for path in entries {

        if is_process_dir(&path) {
            let full_path = path.join(STAT);
            paths.push(full_path);
        }
    }

    Ok(paths)
}

fn is_process_dir(path: &Path) -> bool {
        let path_str = path.to_str().expect("It's not valid UTF-8 path!");

        RE.is_match(path_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_path_is_process_dir() {

        let result = is_process_dir(Path::new("/proc/149066"));
        assert_eq!(result, true);
    }

    #[test]
    fn given_path_is_not_process_dir() {

        let result = is_process_dir(Path::new("/proc/uptime"));
        assert_eq!(result, false);
    }
}
