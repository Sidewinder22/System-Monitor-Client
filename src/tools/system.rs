use std::process::Command;

pub fn get_clk_tck() -> u32 {

    let output = Command::new("getconf")
        .arg("CLK_TCK")
        .output()
        .expect("Failed to execute process");

    let str = String::from_utf8(output.stdout).unwrap();
    str.trim().parse::<u32>().unwrap()
}
