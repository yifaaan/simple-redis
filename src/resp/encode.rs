use super::BulkString;
use super::RespArray;
use super::RespEncode;
use super::RespFrame;
use super::RespMap;
use super::RespNull;
use super::RespNullArray;
use super::RespNullBulkString;
use super::RespSet;
use super::SimpleError;
use super::SimpleString;

const BUF_CAP: usize = 4096;

// - SimpleString: "+OK\r\n"
// - err: "-Error message\r\n"
// - bulk string: "$<length>\r\n<data>\r\n"
// - null bulk string: "$-1\r\n"
// - null: "_\r\n"
// - null array: "*-1\r\n"
// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
// - integer: ":[<+|->]<value>\r\n"
// - boolean: "#<t|f>\r\n"
// - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
// - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
// - set: "~<number-of-elements>\r\n<element-1><element-2>...<element-n>"

impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        match self {
            Self::SimpleSting(s) => s.encode(),
            Self::Array(arr) => arr.encode(),
            Self::Boolean(b) => b.encode(),
            Self::BulkString(s) => s.encode(),
            Self::Double(d) => d.encode(),
            Self::Error(e) => e.encode(),
            Self::Integer(i) => i.encode(),
            Self::Map(m) => m.encode(),
            Self::Null(n) => n.encode(),
            Self::NullArray(n) => n.encode(),
            Self::NullBulkString(n) => n.encode(),
            Self::Set(s) => s.encode(),
        }
    }
}

// - SimpleString: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

// - err: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

// - bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// - null bulk string: "$-1\r\n"
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

// - null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

// - null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("*{}\r\n", self.len()).into_bytes());
        for resp in self.0 {
            buf.extend_from_slice(&resp.encode());
        }
        buf
    }
}

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "-" } else { "+" };
        format!(":{}{}\r\n", sign, self.abs()).into_bytes()
    }
}

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}

// - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let res = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "-" } else { "+" };
            format!(",{}{}\r\n", sign, self.abs())
        };
        res.into_bytes()
    }
}

// - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in self.0 {
            buf.extend_from_slice(&SimpleString(key).encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

// -set: "~<number-of-elements>\r\n<element-1><element-2>...<element-n>"
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for value in self.0 {
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_simple_string_encode() {
        let frame: RespFrame = SimpleString("OK".into()).into();
        assert_eq!(frame.encode(), b"+OK\r\n");
    }

    #[test]
    fn test_integer_encode() {
        let frame: RespFrame = 123.into();
        assert_eq!(frame.encode(), b":+123\r\n");

        let frame: RespFrame = (-123).into();
        assert_eq!(frame.encode(), b":-123\r\n");
    }

    #[test]
    fn test_bulk_string_encode() {
        let frame: RespFrame = BulkString(b"hello".to_vec()).into();
        assert_eq!(frame.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame: RespFrame = RespNullBulkString.into();
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_array_encode() {
        let frame: RespFrame = RespArray(vec![
            BulkString("set".into()).into(),
            BulkString("hello".into()).into(),
            BulkString("world".into()).into(),
        ])
        .into();

        assert_eq!(
            frame.encode(),
            b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let frame: RespFrame = RespNullArray.into();
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_encode() {
        let frame: RespFrame = RespNull.into();
        assert_eq!(frame.encode(), b"_\r\n");
    }

    #[test]
    fn test_boolean_encode() {
        let frame: RespFrame = true.into();
        assert_eq!(frame.encode(), b"#t\r\n");

        let frame: RespFrame = false.into();
        assert_eq!(frame.encode(), b"#f\r\n");
    }

    #[test]
    fn test_double_encode() {
        let frame: RespFrame = 123.456.into();
        assert_eq!(frame.encode(), b",+123.456\r\n");

        let frame: RespFrame = (-123.456).into();
        assert_eq!(frame.encode(), b",-123.456\r\n");

        let frame: RespFrame = 1.23456e+8.into();
        // println!("{}", String::from_utf8_lossy(&frame.encode()));
        assert_eq!(frame.encode(), b",+1.23456e8\r\n");

        let frame: RespFrame = (-1.23456e-9).into();
        // println!("{}", String::from_utf8_lossy(&frame.encode()));
        assert_eq!(frame.encode(), b",-1.23456e-9\r\n");
    }

    #[test]
    fn test_map_encode() {
        let mut map = RespMap(HashMap::new());
        map.insert("hello".into(), BulkString("world".into()).into());

        map.insert("foo".into(), (-123456.789).into());

        let frame: RespFrame = map.into();
        // println!("{:?}", String::from_utf8_lossy(&frame.encode()));
        assert_eq!(
            frame.encode(),
            // b"%2\r\n+hello\r\n$5\r\nworld\r\n+foo\r\n,-123456.789\r\n"
            b"%2\r\n+foo\r\n,-123456.789\r\n+hello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_set_encode() {
        let frame: RespFrame = RespSet(vec![
            RespArray(vec![1234.into(), true.into()]).into(),
            BulkString("world".to_string().into()).into(),
        ])
        .into();
        assert_eq!(
            frame.encode(),
            b"~2\r\n*2\r\n:+1234\r\n#t\r\n$5\r\nworld\r\n"
        );
    }
}
