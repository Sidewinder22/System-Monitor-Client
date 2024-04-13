use super::state::ProcessState;

pub struct ProcessStat {
    pub pid: u32,
    pub comm: String,
    pub state: ProcessState,
    pub utime: u64,
    pub stime: u64,
    pub starttime: u64,
    pub vsize: u64,
}
