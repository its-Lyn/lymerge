use super::log_level::LogLevel;

use std::io;
use std::io::Write;

pub fn log_stdout(
    log_level: LogLevel,
    message: &str,
    new_line: bool
) {
    let message: String = format!("{}: {}", log_level, message);

    if new_line {
        println!("{}", message);
        return;
    }

    println!("{}", message);

    // Flush stdout bc rust is jank...
    io::stdout().flush().expect("Failed to flush stdout.");
}
