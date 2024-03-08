use core::num;

use crate::fs::reader::read_file;

const PROC_STAT_PATH : &str = "/proc/stat";

let mut work : f64;

pub struct CpuLoadStates {
    user : u32,
    nice : u32,
    system : u32,
    idle : u32,
    iowait : u32,
    irq : u32,
    softirr : u32,
    steal : u32,
    guest : u32,
    guest_nice : u32,
}

pub fn get_cpu_average_load() -> String {
    let line = get_cpu_average_load_line();

    let cpu_load = parse_cpu_load_line(&line);

    static PREVIOUS_WORK : f64 = 0.0;
    static PREVIOUS_TOTAL_WORK : f64 = 0.0;

    let work = cpu_load.system + cpu_load.user;
    let total_work = cpu_load.user
        + cpu_load.nice
        + cpu_load.system
        + cpu_load.idle
        + cpu_load.iowait
        + cpu_load.irq
        + cpu_load.softirq
        + cpu_load.steal
        + cpu_load.guest
        + cpu_load.guest_nice;

    let cpu_utilization = (work as f64 / total_work as f64) * 100.0;
    // let cpu_utilization : f64= work as f64 / total_work as f64;
    // let cpu_utilization = total_work - cpu_load.idle;
    // let cpu_utilization : f64= (cpu_load.idle * 100) as f64 / total_work as f64;
    // let cpu_utilization = (cpu_load.idle * 100) / total_work;
    // let cpu_utilization : f64= 100.0 - ((cpu_load.idle * 100) as f64 / total_work as f64);


    // cpu_utilization.to_string()
    format!("CPU_AVERAGE_LOAD: {} END", cpu_utilization)
}

fn get_cpu_average_load_line() -> String {
    let content = read_file(PROC_STAT_PATH);
    let bytes = content.as_bytes();

    for (i, &char) in bytes.iter().enumerate() {
        if b'\n' == char {
            return content[5..i].to_string()
        }
    }

    content.to_string()
}

fn parse_cpu_load_line(line: &str) -> CpuLoadStates {
    let parts : Vec<&str> = line.split(" ").collect();

    let mut numbers : Vec<i32> = Vec::new();
    for part in parts {
        numbers.push(part.parse().unwrap());
    }

    CpuLoadStates {
        user: numbers[0],
        nice : numbers[1],
        system : numbers[2],
        idle : numbers[3],
        iowait : numbers[4],
        irq : numbers[5],
        softirq : numbers[6],
        steal : numbers[7],
        guest : numbers[8],
        guest_nice : numbers[9],
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cpu_load_line_test() {
        let line = "161125 26065 51891 10993268 7731 11249 2827 0 0 0";
        let cpu_load_state = parse_cpu_load_line(line);

        assert_eq!(cpu_load_state.user, 161125);
        assert_eq!(cpu_load_state.nice, 26065);
        assert_eq!(cpu_load_state.system, 51891);
        assert_eq!(cpu_load_state.idle, 10993268);
        assert_eq!(cpu_load_state.iowait, 7731);
        assert_eq!(cpu_load_state.irq, 11249);
        assert_eq!(cpu_load_state.softirq, 2827);
        assert_eq!(cpu_load_state.steal, 0);
        assert_eq!(cpu_load_state.guest, 0);
        assert_eq!(cpu_load_state.guest_nice, 0);
    }
}