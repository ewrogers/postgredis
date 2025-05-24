use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    NullBulkString(),
    Array(Vec<RespValue>),
    NullArray(),
}

impl fmt::Debug for RespValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RespValue::SimpleString(s) => write!(f, "+{}", s),
            RespValue::Error(e) => write!(f, "-{}", e),
            RespValue::Integer(i) => write!(f, ":{}", i),
            RespValue::BulkString(bs) => {
                write!(f, "${},{}", bs.len(), String::from_utf8_lossy(bs))
            }
            RespValue::NullBulkString() => write!(f, "$-1"),
            RespValue::Array(array) => {
                write!(f, "*{}", array.len())?;
                for item in array {
                    write!(f, ",{:?}", item)?;
                }
                Ok(())
            }
            RespValue::NullArray() => write!(f, "*-1"),
        }
    }
}

impl fmt::Display for RespValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RespValue::SimpleString(s) => write!(f, "+{}\r\n", s),
            RespValue::Error(e) => write!(f, "-{}\r\n", e),
            RespValue::Integer(i) => write!(f, ":{}\r\n", i),
            RespValue::BulkString(bs) => {
                write!(f, "${}\r\n{}\r\n", bs.len(), String::from_utf8_lossy(bs))
            }
            RespValue::NullBulkString() => write!(f, "$-1\r\n"),
            RespValue::Array(array) => {
                write!(f, "*{}\r\n", array.len())?;
                for item in array {
                    write!(f, " {}", item)?;
                }
                Ok(())
            }
            RespValue::NullArray() => write!(f, "*-1\r\n"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_simple_string_debug() {
        let value = RespValue::SimpleString("OK".to_string());
        assert_eq!(format!("{:?}", value), "+OK");
    }

    #[test]
    fn test_simple_string_display() {
        let value = RespValue::SimpleString("OK".to_string());
        assert_eq!(format!("{}", value), "+OK\r\n");
    }

    #[test]
    fn test_error_debug() {
        let value = RespValue::Error("ERR invalid syntax".to_string());
        assert_eq!(format!("{:?}", value), "-ERR invalid syntax");
    }

    #[test]
    fn test_error_display() {
        let value = RespValue::Error("ERR invalid syntax".to_string());
        assert_eq!(format!("{}", value), "-ERR invalid syntax\r\n");
    }

    #[test]
    fn test_integer_debug() {
        let value = RespValue::Integer(42);
        assert_eq!(format!("{:?}", value), ":42");
    }

    #[test]
    fn test_integer_display() {
        let value = RespValue::Integer(42);
        assert_eq!(format!("{}", value), ":42\r\n");
    }

    #[test]
    fn test_bulk_string_debug() {
        let value = RespValue::BulkString(b"foobar".to_vec());
        assert_eq!(format!("{:?}", value), "$6,foobar");
    }

    #[test]
    fn test_bulk_string_display() {
        let value = RespValue::BulkString(b"foobar".to_vec());
        assert_eq!(format!("{}", value), "$6\r\nfoobar\r\n");
    }

    #[test]
    fn test_null_bulk_string_debug() {
        let value = RespValue::NullBulkString();
        assert_eq!(format!("{:?}", value), "$-1");
    }

    #[test]
    fn test_null_bulk_string_display() {
        let value = RespValue::NullBulkString();
        assert_eq!(format!("{}", value), "$-1\r\n");
    }

    #[test]
    fn test_array_debug() {
        let value = RespValue::Array(vec![
            RespValue::SimpleString("OK".to_string()),
            RespValue::Integer(42),
        ]);
        assert_eq!(format!("{:?}", value), "*2,+OK,:42");
    }

    #[test]
    fn test_array_display() {
        let value = RespValue::Array(vec![
            RespValue::SimpleString("OK".to_string()),
            RespValue::Integer(42),
        ]);
        assert_eq!(format!("{}", value), "*2\r\n +OK\r\n :42\r\n");
    }

    #[test]
    fn test_null_array_debug() {
        let value = RespValue::NullArray();
        assert_eq!(format!("{:?}", value), "*-1");
    }

    #[test]
    fn test_null_array_display() {
        let value = RespValue::NullArray();
        assert_eq!(format!("{}", value), "*-1\r\n");
    }
}
