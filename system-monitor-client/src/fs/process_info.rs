pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub threads: u32,
}

pub enum ProcessState {
    R,	// Running
    S,	// Sleeping in an interruptible wait
    D, 	// Waiting in uninterruptible disk sleep
    Z, 	// Zombie
    T,	// Stopped (on a signal) or (before Linux 2.6.33) trace stopped
    t,	// Tracing stop (Linux 2.6.33 onward)
    X,	// Dead (from Linux 2.6.0 onward)
    I,  // Idle

    Unknown,  // Unknow
}
