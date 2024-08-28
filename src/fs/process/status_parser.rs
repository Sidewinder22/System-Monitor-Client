use std::{collections::HashSet, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::fs::process::status::ProcessStatus;
use crate::fs::process::state::ProcessState;

const STATE_LINE_SPLIT_POINT: usize = 7;
const NUMBER_REGEX_STR: &str = r"[0-9]*$";
const NAME_REGEX_STR: &str = r"[[:word:]]*$";
const STATE_REGEX_STR: &str = r"[A-Z]{1}";

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(NUMBER_REGEX_STR).unwrap();
    static ref NAME_REGEX: Regex = Regex::new(NAME_REGEX_STR).unwrap();
    static ref STATE_REGEX: Regex = Regex::new(STATE_REGEX_STR).unwrap();
}

pub fn parse(lines: &Vec<String>) -> ProcessStatus {

    let mut pid: Option<u32> = None;
    let mut name: Option<String> = None;
    let mut state: Option<ProcessState> = None;
    let mut threads: Option<u32> = None;

    for line in lines {
        if name.is_none() && line.contains("Name") {
            name = extract_name(line);
            continue;
        } else if state.is_none() && line.contains("State") {
            state = extract_state(line);
            continue;
        } else if pid.is_none() && line.contains("Pid") {
            pid = extract_number(line);
            continue;
        } else if threads.is_none() && line.contains("Threads") {
            threads = extract_number(line);
            continue;
        }

        if name.is_some() &&
            state.is_some() &&
            pid.is_some() &&
            threads.is_some() {
                break;
            }
    }

    ProcessStatus {
        pid: pid.expect("Wrong pid!"),
        name: name.expect("Wrong name!"),
        state: state.expect("Wrong state!"),
        threads: threads.expect("Wrong threads number!")
    }
}

fn extract_name(line: &str) -> Option<String> {

    let caps: HashSet<&str> = NAME_REGEX.find_iter(line)
        .map(|mat| mat.as_str()).collect();

    for cap in caps {
        return Some(String::from(cap));
    }

    None
}

fn extract_state(line: &str) -> Option<ProcessState> {

    let (_, last) = line.split_at(STATE_LINE_SPLIT_POINT);

    let caps: HashSet<&str> = STATE_REGEX.find_iter(last)
    .map(|mat| mat.as_str()).collect();

    for cap in caps {
        return Some(ProcessState::from_str(cap).unwrap());
    }

    None
}

fn extract_number(line: &str) -> Option<u32> {

    let caps: HashSet<&str> = NUMBER_REGEX.find_iter(line)
        .map(|mat| mat.as_str()).collect();

    for cap in caps {
        return Some(cap.parse::<u32>().unwrap());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_apache2_status_lines_parsed_successfully() {

        let lines = [
            "Name:   apache2",
            "Umask:  0022",
            "State:  S (sleeping)",
            "Ngid:   0",
            "Pid:    3673",
            "Threads:        1",
            "SigQ:   0/255927",
            "Seccomp:        2",
        ]
        .map(String::from)
        .to_vec();

        let info = parse(&lines);

        assert_eq!(info.pid, 3673);
        assert_eq!(info.name, "apache2");
        assert_eq!(info.state, ProcessState::S);
        assert_eq!(info.threads, 1);
    }

    #[test]
    fn given_xorg_status_lines_parsed_successfully() {

        let lines = [
            "Name:   Xorg",
            "Umask:  0022",
            "State:  I (idle)",
            "Tgid:   1500",
            "Ngid:   0",
            "Pid:    1500",
            "PPid:   1485",
            "THP_enabled:    1",
            "Threads:        2",
            "SigQ:   0/255927",
        ]
        .map(String::from)
        .to_vec();

        let info = parse(&lines);

        assert_eq!(info.pid, 1500);
        assert_eq!(info.name, "Xorg");
        assert_eq!(info.state, ProcessState::I);
        assert_eq!(info.threads, 2);
    }
}
