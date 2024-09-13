use super::{
    BulkString, RespDecode, RespError, RespFrame, RespNull, RespNullArray, RespNullBulkString,
    SimpleError, SimpleString,
};
use bytes::BytesMut;

impl RespDecode for RespFrame {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => {
                todo!()
            }
            _ => todo!(),
        }
        todo!()
    }
}

// - SimpleString: "+OK\r\n"
impl RespDecode for SimpleString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if buf.len() < 3 {
            return Err(RespError::NotComplete);
        }
        if !buf.starts_with(b"+") {
            return Err(RespError::InvalidFrameType(format!(
                "expect: SimpleString(+), got: {:?}",
                buf
            )));
        }

        // search for "\r\n"
        let mut end = 0;
        for i in 0..buf.len() - 1 {
            if buf[i] == b'\r' && buf[i + 1] == b'\n' {
                end = i;
                break;
            }
        }

        if end == 0 {
            return Err(RespError::NotComplete);
        }

        // split the buffer
        let data = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&data[1..data.len() - 2]);
        Ok(SimpleString(s.into()))
    }
}

// - err: "-Error message\r\n"
impl RespDecode for SimpleError {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if buf.len() < 3 {
            return Err(RespError::NotComplete);
        }

        if !buf.starts_with(b"-") {
            return Err(RespError::InvalidFrameType(format!(
                "expect: SimpleError(-), got: {:?}",
                buf
            )));
        }

        // search for "\r\n"
        let mut end = 0;
        for i in 0..buf.len() - 1 {
            if buf[i] == b'\r' && buf[i + 1] == b'\n' {
                end = i;
                break;
            }
        }
        if end == 0 {
            return Err(RespError::NotComplete);
        }

        // split the buffer
        let data = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&data[1..data.len() - 2]);
        Ok(SimpleError(s.into()))
    }
}

// - bulk string: "$<length>\r\n<data>\r\n"
impl RespDecode for BulkString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if buf.len() < 5 {
            return Err(RespError::NotComplete);
        }

        if !buf.starts_with(b"$") {
            return Err(RespError::InvalidFrameType(format!(
                "expect: BulkString($), got: {:?}",
                buf
            )));
        }

        // search first "\r\n"
        let mut start = 0;
        for i in 0..buf.len() - 1 {
            if buf[i] == b'\r' && buf[i + 1] == b'\n' {
                start = i;
                break;
            }
        }
        if start == 0 {
            return Err(RespError::NotComplete);
        }
        // split the buffer
        let mut data = buf.split_to(start + 2);

        // search second "\r\n"
        let mut end = 0;
        for i in 0..buf.len() - 1 {
            if buf[i] == b'\r' && buf[i + 1] == b'\n' {
                end = i;
                break;
            }
        }
        if end == 0 {
            return Err(RespError::NotComplete);
        }

        data = buf.split_to(end + 2);
        Ok(BulkString(data[..data.len() - 2].into()))
    }
}

// - null bulk string: "$-1\r\n"
impl RespDecode for RespNullBulkString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if &buf[..5] != b"$-1\r\n" {
            return Err(RespError::NotComplete);
        }
        // if !buf.starts_with(b"$") {
        //     return Err(RespError::InvalidFrameType(format!(
        //         "expect: RespNullBulkString($), got: {:?}",
        //         buf
        //     )));
        // }

        // // search for "\r\n"
        // let mut end = 0;
        // for i in 0..buf.len() - 1 {
        //     if buf[i] == b'\r' && buf[i + 1] == b'\n' {
        //         end = i;
        //         break;
        //     }
        // }

        // if end == 0 {
        //     return Err(RespError::NotComplete);
        // }

        // split the buffer
        let _ = buf.split_to(5);
        Ok(RespNullBulkString)
    }
}

// - null: "_\r\n"
impl RespDecode for RespNull {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if &buf[..3] != b"_\r\n" {
            return Err(RespError::NotComplete);
        }
        // if !buf.starts_with(b"_") {
        //     return Err(RespError::InvalidFrameType(format!(
        //         "expect: RespNull(_), got: {:?}",
        //         buf
        //     )));
        // }

        // // search for "\r\n"
        // let mut end = 0;
        // for i in 0..buf.len() - 1 {
        //     if buf[i] == b'\r' && buf[i + 1] == b'\n' {
        //         end = i;
        //         break;
        //     }
        // }

        // if end == 0 {
        //     return Err(RespError::NotComplete);
        // }

        // split the buffer
        let _ = buf.split_to(3);
        Ok(RespNull)
    }
}

// - null array: "*-1\r\n"
impl RespDecode for RespNullArray {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if &buf[..5] != b"*-1\r\n" {
            return Err(RespError::NotComplete);
        }

        // if !buf.starts_with(b"*") {
        //     return Err(RespError::InvalidFrameType(format!(
        //         "expect: RespNullArray(*), got: {:?}",
        //         buf
        //     )));
        // }

        // // search for "\r\n"
        // let mut end = 0;
        // for i in 0..buf.len() - 1 {
        //     if buf[i] == b'\r' && buf[i + 1] == b'\n' {
        //         end = i;
        //         break;
        //     }
        // }

        // if end == 0 {
        //     return Err(RespError::NotComplete);
        // }

        // split the buffer
        let _ = buf.split_to(5);
        Ok(RespNullArray)
    }
}

// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
// impl RespDecode for RespArray {
//     fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {}
// }

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn test_simple_string_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"+OK\r\n");
        let ss = SimpleString::decode(&mut buf)?;
        assert_eq!(ss, SimpleString("OK".into()));

        buf.extend_from_slice(b"+hello\r\n");
        let ss = SimpleString::decode(&mut buf)?;
        assert_eq!(ss, SimpleString("hello".into()));

        buf.extend_from_slice(b"+hello\r");
        let ss = SimpleString::decode(&mut buf);
        assert_eq!(ss.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let ss = SimpleString::decode(&mut buf)?;
        assert_eq!(ss, SimpleString("hello".into()));

        Ok(())
    }

    #[test]
    fn test_simple_error_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"-OK\r\n");
        let ss = SimpleError::decode(&mut buf)?;
        assert_eq!(ss, SimpleError("OK".into()));

        buf.extend_from_slice(b"-hello\r\n");
        let ss = SimpleError::decode(&mut buf)?;
        assert_eq!(ss, SimpleError("hello".into()));

        buf.extend_from_slice(b"-hello\r");
        let ss = SimpleError::decode(&mut buf);
        assert_eq!(ss.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let ss = SimpleError::decode(&mut buf)?;
        assert_eq!(ss, SimpleError("hello".into()));

        Ok(())
    }

    #[test]
    fn test_simple_bulk_string_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$3\r\nabc\r\n");
        let ss = BulkString::decode(&mut buf)?;
        assert_eq!(ss, BulkString("abc".into()));

        buf.extend_from_slice(b"$5\r\nabcde\r\n");
        let ss = BulkString::decode(&mut buf)?;
        assert_eq!(ss, BulkString("abcde".into()));

        buf.extend_from_slice(b"$5\r\nabcde\r");
        let ss = BulkString::decode(&mut buf);
        assert_eq!(ss.unwrap_err(), RespError::NotComplete);

        // buf.put_u8(b'\n');
        // let ss = BulkString::decode(&mut buf)?;
        // assert_eq!(ss, BulkString("abcde".into()));

        Ok(())
    }

    #[test]
    fn test_null_bulk_string_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$-1\r\n");
        let ss = RespNullBulkString::decode(&mut buf)?;
        assert_eq!(ss, RespNullBulkString);

        Ok(())
    }

    #[test]
    fn test_resp_null_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"_\r\n");
        let ss = RespNull::decode(&mut buf)?;
        assert_eq!(ss, RespNull);

        Ok(())
    }

    #[test]
    fn test_resp_null_array_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");
        let ss = RespNullArray::decode(&mut buf)?;
        assert_eq!(ss, RespNullArray);

        Ok(())
    }
}
