use crate::commands::CommandParseError;
use crate::resp::RespValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientCommand {
    Ping(Option<String>),
}

impl TryFrom<RespValue> for ClientCommand {
    type Error = CommandParseError;

    fn try_from(resp: RespValue) -> Result<ClientCommand, Self::Error> {
        match resp {
            RespValue::Array(items) => {
                if items.is_empty() {
                    return Err(CommandParseError::InvalidSyntax);
                }

                let (head, tail) = items.split_first().unwrap();
                let command_name = match head {
                    RespValue::BulkString(bs) => String::from_utf8(bs.clone())
                        .map_err(|_| CommandParseError::InvalidUtf8)?
                        .to_ascii_lowercase(),
                    _ => return Err(CommandParseError::InvalidType),
                };

                match command_name.as_str() {
                    "ping" => {
                        let msg = match tail.len() {
                            0 => None,
                            1 => match &tail[0] {
                                RespValue::BulkString(bs) => Some(
                                    String::from_utf8(bs.clone())
                                        .map_err(|_| CommandParseError::InvalidUtf8)?,
                                ),
                                _ => return Err(CommandParseError::InvalidType),
                            },
                            _ => return Err(CommandParseError::ArityMismatch(command_name)),
                        };
                        Ok(ClientCommand::Ping(msg))
                    }
                    other => Err(CommandParseError::UnknownCommand(other.to_string())),
                }
            }

            // Alternatively allow inline-style strings
            RespValue::SimpleString(s) => match s.to_ascii_lowercase().as_str() {
                "ping" => Ok(ClientCommand::Ping(None)),
                other => Err(CommandParseError::UnknownCommand(other.to_string())),
            },

            _ => Err(CommandParseError::InvalidType),
        }
    }
}
