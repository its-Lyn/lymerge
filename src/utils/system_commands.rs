use std::process;
use std::path::Path;
use std::process::{Command, Stdio};
use sudo::RunningAs;

use std::io::Write;

use crate::logger::log_level::LogLevel;
use crate::logger::logger::{log_stdout, log_string};

pub fn check_os() {
    if !cfg!(unix) || !Path::new("/etc/gentoo-release").exists() {
        log_stdout(LogLevel::Error, "Running lymerge on an unsupported platform, aborting.", true);
        process::exit(1);
    }
}

pub fn ensure_root() {
    if sudo::check() == RunningAs::User {
        log_stdout(LogLevel::Warning, "Lymerge needs to be run as root, elevating.", true);
        sudo::escalate_if_needed().expect(log_string(LogLevel::Error, "Failed to escalate to root.").as_str());
    }
}

pub fn run_command(name: &str, args: Vec<&str>) {
    std::io::stdout().flush().expect("Failed to flush stdout.");

    Command::new(name)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(args)
        .status()
        .expect(log_string(LogLevel::Error, "Failed to run command.").as_str());
}
