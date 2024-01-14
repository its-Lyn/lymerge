use std::process;
use std::path::Path;
use std::process::{Command, Stdio};
use sudo::RunningAs;

use crate::logger::log_level::LogLevel;
use crate::logger::logger::{log_stdout, log_string};

pub fn check_os() {
    if cfg!(unix) && Path::new("/etc/gentoo-release").exists() {
        if sudo::check() == RunningAs::User {
            log_stdout(LogLevel::Warning, "Lyfetch needs to be run as root, elevating.", true);
            sudo::escalate_if_needed().expect("Failed to elevate...");
        }

        return;
    }

    log_stdout(LogLevel::Error, "Running lymerge on an unsupported platform, aborting.", true);
    process::exit(1);
}

pub fn run_command(name: &str, args: Vec<&str>) {
    Command::new(name)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .args(args)
        .output()
        .expect(log_string(LogLevel::Error, "Failed to run command.").as_str());
}
