use std::str::FromStr;

use crate::fs::process::stat::ProcessStat;
use crate::fs::process::state::ProcessState;
use crate::tools::system::get_clk_tck;

pub fn parse(line: &str) -> ProcessStat {

    let mut fields : Vec<&str> = vec![];
    line.split(' ').by_ref().for_each(|value| fields.push(value));

    let clk_tck = get_clk_tck();
    println!("CLK_TCK = {clk_tck}");

    ProcessStat {
        pid: fields[0].parse::<u32>().unwrap(),
        comm: extract_comm(fields[1]),
        state: ProcessState::from_str(fields[2]).unwrap(),
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
        assert_eq!(stat.vsize, 1294336);
    }

}
