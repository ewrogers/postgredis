use crate::commands::CommandParseError;
use crate::resp::RespValue;

pub struct CommandArgs<'a> {
    args: &'a [RespValue],
}

impl<'a> CommandArgs<'a> {
    pub fn new(args: &'a [RespValue]) -> Self {
        Self { args }
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn take_bytes(&self, index: usize) -> Result<&[u8], CommandParseError> {
        match self.args.get(index) {
            Some(RespValue::BulkString(bs)) => Ok(bs),
            Some(RespValue::SimpleString(s)) => Ok(s.as_bytes()),
            _ => Err(CommandParseError::InvalidType),
        }
    }

    pub fn take_string(&self, index: usize) -> Result<String, CommandParseError> {
        let bs = self.take_bytes(index)?;
        String::from_utf8(bs.to_vec()).map_err(|_| CommandParseError::InvalidUtf8)
    }

    pub fn take_opt_string(&self, index: usize) -> Result<Option<String>, CommandParseError> {
        if index < self.len() {
            let s = self.take_string(index)?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }

    pub fn take_int(&self, index: usize) -> Result<i64, CommandParseError> {
        match self.args.get(index) {
            Some(RespValue::Integer(i)) => Ok(*i),
            Some(RespValue::BulkString(bs)) => Ok(String::from_utf8_lossy(bs)
                .parse()
                .map_err(|_| CommandParseError::InvalidType)?),
            Some(RespValue::SimpleString(s)) => {
                Ok(s.parse().map_err(|_| CommandParseError::InvalidType)?)
            }
            _ => Err(CommandParseError::InvalidType),
        }
    }

    pub fn take_opt_int(&self, index: usize) -> Result<Option<i64>, CommandParseError> {
        if index < self.len() {
            let int_value = self.take_int(index)?;
            Ok(Some(int_value))
        } else {
            Ok(None)
        }
    }
}
