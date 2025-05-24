mod args;
pub mod client;
pub mod server;

pub use client::ClientCommand;
use std::fmt;

#[derive(Debug)]
pub enum CommandParseError {
    InvalidSyntax,
    InvalidType,
    InvalidUtf8,
    UnknownCommand(String),
    ArityMismatch(String),
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandParseError::InvalidSyntax => write!(f, "syntax error"),

            CommandParseError::InvalidType => {
                write!(f, "syntax error")
            }
            CommandParseError::InvalidUtf8 => {
                write!(f, "invalid utf-8 string")
            }
            CommandParseError::UnknownCommand(command) => {
                write!(f, "unknown command '{}'", command)
            }
            CommandParseError::ArityMismatch(command) => {
                write!(f, "wrong number of arguments for '{}' command", command)
            }
        }
    }
}
