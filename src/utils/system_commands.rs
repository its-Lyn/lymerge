use std::process;
use std::path::Path;
use std::process::Command;
use sudo::RunningAs;

use crate::logger::log_level::LogLevel;
use crate::logger::logger::log_stdout;

pub fn check_os() {
    if !cfg!(unix) || !Path::new("/etc/gentoo-release").exists() {
        log_stdout(LogLevel::Error, "Running lymerge on an unsupported platform, aborting.", true);
        process::exit(1);
    }
}

pub fn ensure_root() {
    if sudo::check() == RunningAs::User {
        log_stdout(LogLevel::Warning, "Lymerge needs to be ran as root, elevating.", true);
        
        if let Err(_) = sudo::escalate_if_needed() {
            log_stdout(LogLevel::Error, "Failed to escalate, aborting.", true);
            std::process::exit(1);
        }
    }
}

pub fn run_command(name: &str, args: Vec<&str>) { 
    let cmd = Command::new(name)
        .args(args)
        .status();

    if let Err(e) = cmd {
        log_stdout(LogLevel::Error, format!("Command failed to execute: {}, exitting.", e).as_str(), true);
        std::process::exit(1);
    }
}
