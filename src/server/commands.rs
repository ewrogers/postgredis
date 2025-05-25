use crate::resp::RespValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerCommand {
    Pong(Option<String>),
    Error(String),
}

impl Into<RespValue> for ServerCommand {
    fn into(self) -> RespValue {
        match self {
            ServerCommand::Error(msg) => RespValue::Error(msg),
            _ => RespValue::SimpleString("OK".into()),
        }
    }
}
