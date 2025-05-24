#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    NullBulkString(),
    Array(Vec<RespValue>),
    NullArray(),
}

pub struct RespParser {
    buffer: Vec<u8>,
}

impl RespParser {
    pub fn new() -> Self {
        RespParser { buffer: Vec::new() }
    }

    pub fn append(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn parse(&mut self) -> Option<RespValue> {
        if self.buffer.is_empty() {
            return None;
        }

        match self.buffer[0] {
            b'+' => self.parse_simple_string(),
            b'-' => self.parse_error(),
            b':' => self.parse_integer(),
            b'$' => self.parse_bulk_string(),
            b'*' => self.parse_array(),
            _ => None,
        }
    }

    // A simple string starts with '+' and ends with CRLF
    fn parse_simple_string(&mut self) -> Option<RespValue> {
        let end = find_crlf(&self.buffer)?;
        let text = String::from_utf8(self.buffer[1..end].to_vec()).ok()?;
        self.buffer.drain(..end + 2);
        Some(RespValue::SimpleString(text))
    }

    // An error starts with '-' and ends with CRLF
    // It is the same as a simple string with a different start character
    fn parse_error(&mut self) -> Option<RespValue> {
        let end = find_crlf(&self.buffer)?;
        let message = String::from_utf8(self.buffer[1..end].to_vec()).ok()?;
        self.buffer.drain(..end + 2);
        Some(RespValue::Error(message))
    }

    // An integer starts with ':' and ends with CRLF
    // It is the same as a simple string but will be an integer value
    fn parse_integer(&mut self) -> Option<RespValue> {
        let end = find_crlf(&self.buffer)?;
        let payload = String::from_utf8(self.buffer[1..end].to_vec()).ok()?;
        self.buffer.drain(..end + 2);
        let int_value = payload.parse::<i64>().ok()?;
        Some(RespValue::Integer(int_value))
    }

    // A bulk string starts with '$', length, CRLF, data, and ends with another CRLF
    // It is used to transmit a binary-safe string up to 512MB
    fn parse_bulk_string(&mut self) -> Option<RespValue> {
        let len_end = find_crlf(&self.buffer)?;
        let len_string = String::from_utf8(self.buffer[1..len_end].to_vec()).ok()?;
        let len: isize = len_string.parse().ok()?;
        let header_bytes = len_end + 2;

        if len < 0 {
            self.buffer.drain(..header_bytes);
            return Some(RespValue::NullBulkString());
        }

        let total_size = header_bytes + (len as usize) + 2;
        if self.buffer.len() < total_size {
            return None;
        }

        let data = self.buffer[header_bytes..header_bytes + (len as usize)].to_vec();
        if &self.buffer[header_bytes..header_bytes + len_end] != b"\r\n" {
            return None;
        }
        self.buffer.drain(..total_size);
        Some(RespValue::BulkString(data))
    }

    // An array starts with '*', count, CRLF, and then each RESP type serialized for count
    fn parse_array(&mut self) -> Option<RespValue> {
        let header_end = find_crlf(&self.buffer)?;
        let count_string = String::from_utf8(self.buffer[1..header_end].to_vec()).ok()?;
        let count: isize = count_string.parse().ok()?;
        let mut offset = header_end + 2;

        if count < 0 {
            self.buffer.drain(..offset);
            return Some(RespValue::NullArray());
        }

        let mut items = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let slice = self.buffer[offset..].to_vec();
            let slice_len = slice.len();

            let mut item_parser = RespParser { buffer: slice };

            if let Some(item) = item_parser.parse() {
                let consumed = slice_len - item_parser.buffer.len();
                offset += consumed;
                items.push(item);
            }
        }

        self.buffer.drain(..offset);
        Some(RespValue::Array(items))
    }
}

fn find_crlf(buffer: &[u8]) -> Option<usize> {
    buffer.windows(2).position(|window| window == b"\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parses all possible values from an input buffer
    fn parse_all(buffer: &[u8]) -> Vec<RespValue> {
        let mut parser = RespParser::new();
        parser.append(buffer);

        let mut results = Vec::new();
        while let Some(item) = parser.parse() {
            results.push(item);
        }
        results
    }

    #[test]
    fn test_simple_string() {
        let got = parse_all(b"+OK\r\n");
        assert_eq!(got, vec![RespValue::SimpleString("OK".into())]);
    }

    #[test]
    fn test_error() {
        let got = parse_all(b"-ERR Something went wrong\r\n");
        assert_eq!(
            got,
            vec![RespValue::Error("ERR Something went wrong".into())]
        );
    }

    #[test]
    fn test_integer() {
        let got = parse_all(b":12345");
        assert_eq!(got, vec![RespValue::Integer(12345)]);
    }

    #[test]
    fn test_bulk_string() {
        let got = parse_all(b"$6\r\nfoobar\r\n");
        assert_eq!(got, vec![RespValue::BulkString(b"foobar".to_vec())]);
    }

    #[test]
    fn test_bulk_string_empty() {
        let got = parse_all(b"$0\r\n\r\n");
        assert_eq!(got, vec![RespValue::BulkString(b"".to_vec())]);
    }

    #[test]
    fn test_null_bulk_string() {
        let got = parse_all(b"$-1\r\n");
        assert_eq!(got, vec![RespValue::NullBulkString()]);
    }

    #[test]
    fn test_array() {
        let got = parse_all(b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
        assert_eq!(
            got,
            vec![RespValue::Array(vec![
                RespValue::BulkString(b"foo".to_vec()),
                RespValue::BulkString(b"bar".to_vec()),
            ])]
        );
    }

    #[test]
    fn test_array_empty() {
        let got = parse_all(b"*0\r\n");
        assert_eq!(got, vec![RespValue::Array(Vec::new())]);
    }

    #[test]
    fn test_null_array() {
        let got = parse_all(b"*-1\r\n");
        assert_eq!(got, vec![RespValue::NullArray()]);
    }
}
