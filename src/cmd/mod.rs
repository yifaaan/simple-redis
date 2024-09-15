mod hmap;
mod map;

use crate::{
    backend::Backend,
    resp::{RespArray, RespError, RespFrame, SimpleString},
};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use thiserror::Error;

lazy_static! {
    static ref RESP_OK: RespFrame = RespFrame::SimpleSting(SimpleString("OK".into()));
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(self, backend: &Backend) -> RespFrame;
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    RespError(#[from] RespError),

    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[enum_dispatch(CommandExecutor)]
#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    HGetAll(HGetAll),
    UnknownCmd(UnknownCmd),
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

#[derive(Debug)]
pub struct UnknownCmd;

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;
    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(array) => array.try_into(),
            _ => Err(CommandError::InvalidCommand(format!(
                "COmmand must be an Array"
            ))),
        }
    }
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match value.first() {
            Some(RespFrame::BulkString(cmd)) => match cmd.as_slice() {
                b"get" => Ok(Command::Get(value.try_into()?)),
                b"set" => Ok(Command::Set(value.try_into()?)),
                b"hget" => Ok(Command::HGet(value.try_into()?)),
                b"hset" => Ok(Command::HSet(value.try_into()?)),
                b"hgetall" => Ok(Command::HGetAll(value.try_into()?)),
                _ => Ok(Command::UnknownCmd(UnknownCmd)),
            },
            _ => Err(CommandError::InvalidCommand(format!(
                "Command mut have a BulkString as the first argument"
            ))),
        }
    }
}

// enum_dispatch 的功能

// impl CommandExecutor for Command {
//     fn execute(self, backend: &Backend) -> RespFrame {
//         match self {
//             Self::Get(get) => get.execute(&backend),
//             Self::Set(set) => set.execute(&backend),
//             Self::HGet(hget) => hget.execute(&backend),
//             Self::HSet(hset) => hset.execute(&backend),
//             Self::HGetAll(hgetall) => hgetall.execute(&backend),
//         }
//     }
// }

fn validate_command(
    value: &RespArray,
    names: &[&'static str],
    n_args: usize,
) -> Result<(), CommandError> {
    if value.len() != n_args + names.len() {
        return Err(CommandError::InvalidArgument(format!(
            "{} command must have exactly {} argument",
            names.join(" "),
            n_args
        )));
    }
    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref cmd) => {
                if cmd.to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "Invalid command: expected {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd)
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(format!(
                    "{} command must have a BulkString as the first argument",
                    name
                )));
            }
        }
    }

    Ok(())
}

fn extract_args(value: RespArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value.0.into_iter().skip(start).collect())
}

impl CommandExecutor for UnknownCmd {
    fn execute(self, backend: &Backend) -> RespFrame {
        RESP_OK.clone()
    }
}
