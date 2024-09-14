mod get;
use crate::resp::RespFrame;

pub trait CommandExecutor {
    fn execute(self) -> RespFrame;
}

pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    HGetAll(HGetAll),
}

#[derive(Debug)]
pub struct Get {
    key: String,
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGet {
    key: String,
    field: String,
}

#[derive(Debug)]
pub struct HSet {
    key: String,
    field: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGetAll {
    key: String,
}
