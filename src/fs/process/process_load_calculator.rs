use std::path::PathBuf;

use crate::fs::reader::read_file;
use crate::fs::process::stat_parser;
use crate::fs::cpu::cpu_load_provider::CpuLoadProvider;

const STAT: &str = "stat";

pub struct ProcessLoadCalculator {
    previous_utime: u64,
    previous_stime: u64,
    previous_total_cpu_work: u64,
    num_of_cores: usize,
}

impl ProcessLoadCalculator {
    pub fn new() -> ProcessLoadCalculator {
        ProcessLoadCalculator {
            previous_utime: 0,
            previous_stime: 0,
            previous_total_cpu_work: 0,
            num_of_cores: num_cpus::get(),
        }
    }
    
    pub fn get_process_load(&mut self, path : &PathBuf) -> f64 {

        let full_path = path.join(STAT);

        let line = read_file(&full_path.display().to_string());
        if line.is_empty() {
            return 0.0
        }

        let stat = stat_parser::parse(&line);

        let total_cpu_work = CpuLoadProvider::get_total_cpu_work();
        let time_diff = total_cpu_work - self.previous_total_cpu_work;
        let utime = 100.0 * (stat.utime - self.previous_utime) as f64 / time_diff as f64;
        let stime = 100.0 * (stat.stime - self.previous_stime) as f64 / time_diff as f64;

        self.previous_utime = stat.utime;
        self.previous_stime = stat.stime;
        self.previous_total_cpu_work = total_cpu_work;

        self.num_of_cores as f64 * (utime + stime)
    }
}
