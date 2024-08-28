use std::str::FromStr;

#[derive(PartialEq, Debug)]
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

impl FromStr for ProcessState {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(ProcessState::R),
            "S" => Ok(ProcessState::S),
            "D" => Ok(ProcessState::D),
            "Z" => Ok(ProcessState::Z),
            "T" => Ok(ProcessState::T),
            "t" => Ok(ProcessState::t),
            "X" => Ok(ProcessState::X),
            "I" => Ok(ProcessState::I),
            _   => Ok(ProcessState::Unknown),
        }
    }
}

pub fn process_state_to_str(process_state : &ProcessState) -> String {
    match process_state {
        ProcessState::R => String::from("R"),
        ProcessState::S => String::from("S"),
        ProcessState::D => String::from("D"),
        ProcessState::Z => String::from("Z"),
        ProcessState::T => String::from("T"),
        ProcessState::t => String::from("t"),
        ProcessState::X => String::from("X"),
        ProcessState::I => String::from("I"),
        ProcessState::Unknown => String::from("Unknown"),
    }
}
