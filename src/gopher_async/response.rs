use bytes::BytesMut;
use tokio::codec::{Decoder, Encoder, LinesCodec};

use crate::errors::Error;

use super::types::ItemType;

#[derive(Debug)]
pub enum Response {
    Menu(Vec<MenuEntry>),
    TextFile,
    BinaryFile,
}

#[derive(Debug)]
pub enum MenuEntry {
    Information(String),
    Link(ItemType, String, String),
}

impl Response {
    pub fn from_buf(buf: Vec<u8>) -> Result<Self, Error> {
        let string = String::from_utf8(buf).map_err(Error::from)?;
        let lines = string.lines();
        let mut entries = Vec::new();
        let mut current = Vec::new();

        for line in lines {
            let line_ty = ItemType::decode(line.as_bytes()[0]);
            let rest = &line[1..];

            let parts = rest.split("\t").collect::<Vec<_>>();

            // join the information strings together
            if let ItemType::Other(_) = line_ty {
                current.push(parts[0]);
                continue;
            } else if !current.is_empty() {
                entries.push(MenuEntry::Information(current.join("\n")));
                current.clear();
            }

            // TODO
            match line_ty {
                File => entries.push(MenuEntry::Link(
                    line_ty,
                    parts[0].to_owned(),
                    parts[1].to_owned(),
                )),
                _ => unimplemented!("unimplemented type {:?}", line_ty),
            }
        }

        entries.push(MenuEntry::Information(current.join("\n")));
        Ok(Response::Menu(entries))
    }
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
        info!("Decoding: {:?}", bytes);
        Ok(None)
    }
}
