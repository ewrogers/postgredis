use crate::resp::RespValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerCommand {
    Pong(Option<String>),
}

impl Into<RespValue> for ServerCommand {
    fn into(self) -> RespValue {
        RespValue::SimpleString("OK".into())
    }
}
