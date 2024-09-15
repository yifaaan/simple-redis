use crate::{
    backend::{self, Backend},
    cmd::{Command, CommandExecutor},
    resp::{RespDecode, RespEncode, RespError, RespFrame},
};
use anyhow::{Ok, Result};
use bytes::BufMut;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;
#[derive(Debug)]
struct RespFrameCodec;

#[derive(Debug)]
struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

#[derive(Debug)]
struct RedisResponse {
    frame: RespFrame,
}

pub async fn stream_handler(mut stream: TcpStream, backend: Backend) -> Result<()> {
    let mut framed = Framed::new(stream, RespFrameCodec);
    loop {
        match framed.next().await {
            Some(core::result::Result::Ok(frame)) => {
                info!("Received frame: {:?}", frame);
                let request = RedisRequest {
                    frame,
                    backend: backend.clone(),
                };
                let response = request_handler(request).await?;
                framed.send(response.frame).await?;
            }
            Some(Err(e)) => return Err(e.into()),
            None => return Ok(()),
        }
    }
}

async fn request_handler(request: RedisRequest) -> Result<RedisResponse> {
    let (frame, backend) = (request.frame, request.backend);
    let cmd: Command = frame.try_into()?;
    info!("Executing command: {:?}", cmd);
    let frame = cmd.execute(&backend);
    Ok(RedisResponse { frame })
}

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;
    fn encode(
        &mut self,
        item: RespFrame,
        dst: &mut bytes::BytesMut,
    ) -> std::result::Result<(), Self::Error> {
        let encoded = item.encode();
        dst.put(encoded.as_ref());
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Error = anyhow::Error;
    type Item = RespFrame;

    fn decode(
        &mut self,
        src: &mut bytes::BytesMut,
    ) -> std::result::Result<Option<Self::Item>, Self::Error> {
        match RespFrame::decode(src) {
            core::result::Result::Ok(frame) => Ok(Some(frame)),
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
