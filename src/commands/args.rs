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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let args = vec![RespValue::SimpleString("hello".to_string())];
        let cmd_args = CommandArgs::new(&args);
        assert_eq!(cmd_args.len(), 1);
    }

    #[test]
    fn test_len() {
        let args = vec![
            RespValue::SimpleString("foo".to_string()),
            RespValue::SimpleString("bar".to_string()),
        ];
        let cmd_args = CommandArgs::new(&args);
        assert_eq!(cmd_args.len(), 2);
    }

    #[test]
    fn test_take_bytes() {
        let args = vec![
            RespValue::BulkString(b"bulk".to_vec()),
            RespValue::SimpleString("simple".to_string()),
            RespValue::Integer(42),
        ];
        let cmd_args = CommandArgs::new(&args);

        assert_eq!(cmd_args.take_bytes(0).unwrap(), b"bulk");
        assert_eq!(cmd_args.take_bytes(1).unwrap(), b"simple");
        assert!(cmd_args.take_bytes(2).is_err());
    }

    #[test]
    fn test_take_string() {
        let args = vec![
            RespValue::BulkString(b"valid".to_vec()),
            RespValue::BulkString(vec![0xFF, 0xFF]),
        ];
        let cmd_args = CommandArgs::new(&args);

        assert_eq!(cmd_args.take_string(0).unwrap(), "valid");
        assert!(cmd_args.take_string(1).is_err());
    }

    #[test]
    fn test_take_opt_string() {
        let args = vec![RespValue::SimpleString("test".to_string())];
        let cmd_args = CommandArgs::new(&args);

        assert_eq!(
            cmd_args.take_opt_string(0).unwrap(),
            Some("test".to_string())
        );
        assert_eq!(cmd_args.take_opt_string(1).unwrap(), None);
    }

    #[test]
    fn test_take_int() {
        let args = vec![
            RespValue::Integer(42),
            RespValue::BulkString(b"123".to_vec()),
            RespValue::SimpleString("456".to_string()),
            RespValue::SimpleString("invalid".to_string()),
        ];
        let cmd_args = CommandArgs::new(&args);

        assert_eq!(cmd_args.take_int(0).unwrap(), 42);
        assert_eq!(cmd_args.take_int(1).unwrap(), 123);
        assert_eq!(cmd_args.take_int(2).unwrap(), 456);
        assert!(cmd_args.take_int(3).is_err());
    }

    #[test]
    fn test_take_opt_int() {
        let args = vec![RespValue::Integer(42)];
        let cmd_args = CommandArgs::new(&args);

        assert_eq!(cmd_args.take_opt_int(0).unwrap(), Some(42));
        assert_eq!(cmd_args.take_opt_int(1).unwrap(), None);
    }
}
