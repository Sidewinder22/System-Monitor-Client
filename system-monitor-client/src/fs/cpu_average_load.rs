use crate::fs::cpu_load::CpuLoad;
use crate::fs::reader::read_file;

const PROC_STAT_PATH: &str = "/proc/stat";

pub struct CpuAverageLoad {
    previous_cpu_load: CpuLoad,
}

impl CpuAverageLoad {
    pub fn new() -> CpuAverageLoad {
        CpuAverageLoad {
            previous_cpu_load: CpuLoad {
                user: 0,
                nice: 0,
                system: 0,
                idle: 0,
                iowait: 0,
                irq: 0,
                softirq: 0,
                steal: 0,
                guest: 0,
                guest_nice: 0,
            },
        }
    }

    pub fn get_cpu_average_load(&mut self) -> String {
        let line = get_cpu_average_load_line();
        let cpu_load = parse_cpu_load_line(&line);

        let work = cpu_load.system + cpu_load.user;
        let previous_work = self.previous_cpu_load.system + self.previous_cpu_load.user;
        let total_work = calculate_total_work(&cpu_load);
        let previous_total_work = calculate_total_work(&self.previous_cpu_load);

        let work_over_period = work - previous_work;
        let total_work_over_period = total_work - previous_total_work;

        let cpu_utilization = (work_over_period as f64 / total_work_over_period as f64) * 100.0;

        self.previous_cpu_load = cpu_load;

        format!("CPU_AVERAGE_LOAD: {} END", cpu_utilization)
    }
}

fn calculate_total_work(cpu_load: &CpuLoad) -> u64 {
    cpu_load.user
        + cpu_load.nice
        + cpu_load.system
        + cpu_load.idle
        + cpu_load.iowait
        + cpu_load.irq
        + cpu_load.softirq
        + cpu_load.steal
        + cpu_load.guest
        + cpu_load.guest_nice
}

fn get_cpu_average_load_line() -> String {
    let content = read_file(PROC_STAT_PATH);
    let bytes = content.as_bytes();

    for (i, &char) in bytes.iter().enumerate() {
        if b'\n' == char {
            return content[5..i].to_string();
        }
    }

    content.to_string()
}

fn parse_cpu_load_line(line: &str) -> CpuLoad {
    let parts: Vec<&str> = line.split(" ").collect();

    let mut numbers: Vec<u64> = Vec::new();
    for part in parts {
        numbers.push(part.parse().unwrap());
    }

    CpuLoad {
        user: numbers[0],
        nice: numbers[1],
        system: numbers[2],
        idle: numbers[3],
        iowait: numbers[4],
        irq: numbers[5],
        softirq: numbers[6],
        steal: numbers[7],
        guest: numbers[8],
        guest_nice: numbers[9],
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
