#![windows_subsystem = "windows"]

use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let mins = Duration::from_secs(19 * 60);
    let mut cmd = Command::new("alert.exe");

    loop {
        let process = cmd.spawn().expect("Couldn't spawn process.");
        println!("{:?}", process.wait_with_output().unwrap());
        thread::sleep(mins);
    }
}
