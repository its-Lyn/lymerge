use std::fmt;

// COLOUR  CODE
// 
// ERROR   91
// WARNING 93
// INFO    96
// NC      00

// COLOUR FORMAT
// \x1b[<colour>m

fn format_colour(string: &str, colour: i32) -> String {
    format!("\x1b[{}m{}\x1b[00m", colour, string)
}

pub enum LogLevel {
    Info,
    Error,
    Warning
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
           LogLevel::Info => write!(f, "{}", format_colour("INFO", 96)),
           LogLevel::Warning => write!(f, "{}", format_colour("WARNING", 93)),
           LogLevel::Error => write!(f, "{}", format_colour("ERROR", 91))
        } 
    }
}
