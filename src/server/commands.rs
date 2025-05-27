use crate::resp::RespValue;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerCommand {
    Pong(Option<String>),
    Response(RespValue),
    Ok,
    Error(String),
}

#[allow(unreachable_patterns)]
impl Into<RespValue> for ServerCommand {
    fn into(self) -> RespValue {
        match self {
            ServerCommand::Pong(message) => match message {
                Some(msg) => {
                    let output = format!("PONG {}", msg);
                    RespValue::BulkString(output.into_bytes())
                }
                None => RespValue::SimpleString("PONG".into()),
            },
            ServerCommand::Response(value) => value,
            ServerCommand::Ok => RespValue::SimpleString("OK".into()),
            ServerCommand::Error(message) => RespValue::Error(message),
            _ => RespValue::SimpleString("OK".into()),
        }
    }
}
