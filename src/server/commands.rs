use crate::resp::RespValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerCommand {
    Pong(Option<String>),
    Response(RespValue),
    Error(String),
}

impl Into<RespValue> for ServerCommand {
    fn into(self) -> RespValue {
        match self {
            ServerCommand::Pong(message) => match message {
                Some(message) => {
                    let output = format!("PONG {}", message);
                    RespValue::BulkString(output.into_bytes())
                }
                None => RespValue::SimpleString("PONG".into()),
            },
            ServerCommand::Response(value) => value,
            ServerCommand::Error(message) => RespValue::Error(message),
            _ => RespValue::SimpleString("OK".into()),
        }
    }
}
