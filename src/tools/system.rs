use std::process::Command;

pub fn get_clk_tck() -> u32 {

    let output = Command::new("getconf")
        .arg("CLK_TCK")
        .output()
        // .spawn()
        .expect("Failed to execute process");

    let str = String::from_utf8(output.stdout).unwrap();
    let str = str.trim();

    str.parse::<u32>().unwrap()
}
