use crate::fs::process::state::ProcessState;

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub threads: u32,
}
