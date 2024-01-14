use std::fmt;

pub enum LogLevel {
    Info,
    Error,
    Warning
}

// TODO: implement colouring...
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           LogLevel::Info => write!(f, "INFO"),
           LogLevel::Warning => write!(f, "WARNING"),
           LogLevel::Error => write!(f, "ERROR")
        } 
    }
}
