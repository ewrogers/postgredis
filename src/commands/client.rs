use crate::commands::{CommandArgs, CommandParseError};
use crate::resp::RespValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientCommand {
    Ping(Option<String>),
}

impl TryFrom<RespValue> for ClientCommand {
    type Error = CommandParseError;

    fn try_from(resp: RespValue) -> Result<ClientCommand, Self::Error> {
        if let RespValue::Array(mut array) = resp {
            if array.is_empty() {
                return Err(CommandParseError::InvalidSyntax);
            }

            let command_name = match array.remove(0) {
                RespValue::BulkString(bs) => String::from_utf8(bs)
                    .map_err(|_| CommandParseError::InvalidUtf8)?
                    .to_ascii_lowercase(),
                _ => return Err(CommandParseError::InvalidType),
            };

            let args = CommandArgs::new(&array);
            match command_name.as_str() {
                // PING [message]
                "ping" => {
                    if args.len() > 1 {
                        return Err(CommandParseError::ArityMismatch(command_name));
                    }
                    let message = args.take_opt_string(0)?;
                    Ok(ClientCommand::Ping(message))
                }
                other => Err(CommandParseError::UnknownCommand(other.to_string())),
            }
        } else {
            Err(CommandParseError::InvalidSyntax)
        }
    }
}
