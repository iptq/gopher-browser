use bytes::BytesMut;
use tokio::codec::{Decoder, Encoder, LinesCodec};

use super::errors::Error;
use super::types::ItemType;

pub enum Response {
    Menu(Vec<MenuEntry>),
    TextFile,
    BinaryFile,
}

pub enum MenuEntry {
    Information(String),
    Link(ItemType, String, String),
}

pub struct ResponseCodec(ItemType);

impl ResponseCodec {
    pub fn new(item_type: ItemType) -> Self {
        ResponseCodec(item_type)
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
