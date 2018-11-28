use bytes::{BufMut, BytesMut};
use tokio::codec::{Decoder, Encoder};

use std::io;

pub struct ProtoCodec;

impl Decoder for ProtoCodec {
    type Item = u32;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buf.len() >= 4 {
            let array = [buf[0], buf[1], buf[2], buf[3]];
            let n = u32::from_le_bytes(array);
            buf.split_to(4);
            return Ok(Some(n));
        } else {
            return Ok(None);
        }
    }
}

impl Encoder for ProtoCodec {
    type Item = u32;
    type Error = io::Error;

    fn encode(&mut self, packet: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.reserve(4);
        buf.put_u32_le(packet);

        Ok(())
    }
}
