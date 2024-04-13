use std::str::FromStr;

use crate::fs::process::stat::ProcessStat;
use crate::fs::process::state::ProcessState;

pub fn parse(line: &str) -> ProcessStat {

    let mut fields : Vec<&str> = vec![];
    line.split(' ').by_ref().for_each(|value| fields.push(value));

    ProcessStat {
        pid: fields[0].parse::<u32>().unwrap(),
        comm: extract_comm(fields[1]),
        state: ProcessState::from_str(fields[2]).unwrap(),
        utime: fields[13].parse::<u64>().unwrap(),
        stime: fields[14].parse::<u64>().unwrap(),
        starttime: fields[21].parse::<u64>().unwrap(),
        vsize: fields[22].parse::<u64>().unwrap(),
    }
}

fn extract_comm(field: &str) -> String {
    let len = field.len();
    let str = field.to_string();
    str[1..len-1].to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_init_stat_line_parsed_successfully() {

        let line = String::from("909 (init) S 1 909 909 0 -1 4194624 13 0 0 0 0 0 0 0 20 0 1 0 28226 1294336 92 18446744073709551615 1 1 0 0 0 0 65536 2147024638 65536 0 0 0 17 5 0 0 0 0 0 0 0 0 0 0 0 0 0");

        let stat = parse(&line);

        assert_eq!(stat.pid, 909);
        assert_eq!(stat.comm, "init");
        assert_eq!(stat.state, ProcessState::S);
        assert_eq!(stat.utime, 0);
        assert_eq!(stat.stime, 0);
        assert_eq!(stat.starttime, 28226);
        assert_eq!(stat.vsize, 1294336);
    }

    #[test]
    fn given_chrome_stat_line_parsed_successfully() {
        let line = String::from("19071 (chrome) S 1588 1202 1202 0 -1 4194368 48486 0 2 0 106 24 0 0 20 0 32 0 8557978 1215944785920 48075 18446744073709551615 94002235985920 94002420538864 140730014952128 0 0 0 0 4098 1098993405 0 0 0 17 6 0 0 0 0 0 94002430128128 94002430757448 94002456518656 140730014954996 140730014955126 140730014955126 140730014957534 0");

        let stat = parse(&line);

        assert_eq!(stat.pid, 19071);
        assert_eq!(stat.comm, "chrome");
        assert_eq!(stat.state, ProcessState::S);
        assert_eq!(stat.utime, 106);
        assert_eq!(stat.stime, 24);
        assert_eq!(stat.starttime, 8557978);
        assert_eq!(stat.vsize, 1215944785920);
    }
}
