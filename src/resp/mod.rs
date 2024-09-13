mod decode;
mod encode;

use bytes::BytesMut;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

pub enum RespFrame {
    SimpleSting(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    Array(RespArray),
    Null(RespNull),
    NullArray(RespNullArray),
    NullBulkString(RespNullBulkString),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

pub struct SimpleString(String);

impl Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SimpleError(String);

impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct BulkString(Vec<u8>);

impl Deref for BulkString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct RespArray(Vec<RespFrame>);

impl Deref for RespArray {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RespMap(HashMap<String, RespFrame>);

impl Deref for RespMap {
    type Target = HashMap<String, RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RespSet(HashSet<RespFrame>);

impl Deref for RespSet {
    type Target = HashSet<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct RespNull;
pub struct RespNullArray;
pub struct RespNullBulkString;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

impl RespDecode for BytesMut {
    fn decode(buf: Self) -> Result<RespFrame, String> {
        todo!()
    }
}
