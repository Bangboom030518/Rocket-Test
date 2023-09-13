use std::str;
use std::process::Command;

pub fn execute_command(command: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args(["-c", command])
            .output()
            .expect("failed to execute process")
    };
    str::from_utf8(&output.stdout).unwrap().to_string()
}