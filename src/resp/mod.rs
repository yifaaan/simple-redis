mod decode;
mod encode;

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use std::collections::{BTreeSet, HashMap};
use std::ops::{Deref, DerefMut};

#[enum_dispatch(RespEncode)]
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

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct RespSet(Vec<RespFrame>);

impl Deref for RespSet {
    type Target = Vec<RespFrame>;
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

impl From<SimpleString> for RespFrame {
    fn from(value: SimpleString) -> Self {
        Self::SimpleSting(value)
    }
}

impl From<SimpleError> for RespFrame {
    fn from(value: SimpleError) -> Self {
        Self::Error(value)
    }
}
impl From<i64> for RespFrame {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<BulkString> for RespFrame {
    fn from(value: BulkString) -> Self {
        Self::BulkString(value)
    }
}
impl From<RespArray> for RespFrame {
    fn from(value: RespArray) -> Self {
        Self::Array(value)
    }
}
impl From<RespNull> for RespFrame {
    fn from(value: RespNull) -> Self {
        Self::Null(value)
    }
}

impl From<RespNullArray> for RespFrame {
    fn from(value: RespNullArray) -> Self {
        Self::NullArray(value)
    }
}
impl From<RespNullBulkString> for RespFrame {
    fn from(value: RespNullBulkString) -> Self {
        Self::NullBulkString(value)
    }
}
impl From<bool> for RespFrame {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<f64> for RespFrame {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}
impl From<RespMap> for RespFrame {
    fn from(value: RespMap) -> Self {
        Self::Map(value)
    }
}

impl From<RespSet> for RespFrame {
    fn from(value: RespSet) -> Self {
        Self::Set(value)
    }
}
#[enum_dispatch]
pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

impl RespDecode for BytesMut {
    fn decode(buf: Self) -> Result<RespFrame, String> {
        todo!()
    }
}
