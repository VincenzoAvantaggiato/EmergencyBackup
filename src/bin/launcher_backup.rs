#![windows_subsystem = "windows"]
use std::env;
use std::process::{Command, Stdio};

fn main() {

    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();

    let app_path = wd.join("Group13");
    let out = Command::new(app_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run Group13");

    Command::new("killall")
        .arg("Terminal")
        .spawn()
        .expect("Failed to run killall terminal");

    std::process::exit(0);
}
