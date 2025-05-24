pub mod client;
pub mod server;

pub use client::ClientCommand;

#[derive(Debug)]
pub enum CommandParseError {
    UnknownCommand(String),
    InvalidType,
    ArityMismatch,
    InvalidUtf8,
}
