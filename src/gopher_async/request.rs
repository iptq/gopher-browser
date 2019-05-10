use bytes::{BufMut, BytesMut};
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::codec::{Decoder, Encoder, LinesCodec};
use url::Url;

use crate::errors::Error;

use super::types::ItemType;

#[derive(Debug)]
pub struct Request {
    pub addr: SocketAddr,
    pub item_type: ItemType,
    pub resource: String,
}

impl Request {
    pub fn from_url(url: Url) -> Result<Self, Error> {
        let addr = url
            .with_default_port(|_| Ok(70))
            .and_then(|host_and_port| host_and_port.to_socket_addrs())
            .map(|mut iter| iter.next().unwrap())?;

        let (item_type, resource) = url
            .path_segments()
            .and_then(|mut iter| iter.next().map(|item| (item, iter)))
            .and_then(|(first_arg, iter)| {
                if first_arg.len() == 0 {
                    return None;
                }
                let ty = ItemType::decode(first_arg.as_bytes()[0]);
                let rest = iter.collect::<Vec<_>>().join("/");
                Some((ty, rest))
            })
            .unwrap_or_else(|| (ItemType::Dir, String::new()));

        Ok(Request {
            addr,
            item_type,
            resource,
        })
    }
}

pub struct RequestCodec;

impl RequestCodec {
    pub fn new() -> Self {
        RequestCodec
    }
}

impl Encoder for RequestCodec {
    type Item = Request;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        // TODO: do this
        // Before writing to the buffer, ensure that there is enough remaining capacity by calling my_bytes.remaining_mut().

        bytes.put(item.resource);
        bytes.put("\n");
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
