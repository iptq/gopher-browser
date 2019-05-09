use bytes::BytesMut;
use tokio::codec::{Decoder, Encoder, LinesCodec};

use super::errors::Error;

pub enum Response {
    Hello,
}

pub struct ResponseCodec(LinesCodec);

impl ResponseCodec {
    pub fn new() -> Self {
        let inner = LinesCodec::new();
        ResponseCodec(inner)
    }
}

impl Encoder for ResponseCodec {
    type Item = Response;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Decoder for ResponseCodec {
    type Item = Response;
    type Error = Error;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}
