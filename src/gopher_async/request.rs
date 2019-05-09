use bytes::BytesMut;
use tokio::codec::{Decoder, Encoder, LinesCodec};

use crate::gopher::errors::Error;

pub struct Request {
    pub selector: String,
    pub query: Option<String>,
}

pub struct RequestCodec(LinesCodec);

impl RequestCodec {
    pub fn new() -> Self {
        let inner = LinesCodec::new();
        RequestCodec(inner)
    }
}

impl Encoder for RequestCodec {
    type Item = Request;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Decoder for RequestCodec {
    type Item = Request;
    type Error = Error;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}
