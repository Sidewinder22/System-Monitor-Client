use super::state::ProcessState;

pub struct ProcessStat {
    pub pid: u32,
    pub comm: String,
    pub state: ProcessState,
    pub vsize: u64,
}
