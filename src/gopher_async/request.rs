use bytes::BytesMut;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::codec::{Decoder, Encoder, LinesCodec};
use url::Url;

use super::errors::Error;
use super::types::ItemType;

pub struct Request {
    pub addr: SocketAddr,
    pub resource: Option<(ItemType, String)>,
}

impl Request {
    pub fn from_url(url: Url) -> Result<Self, Error> {
        let addr = url
            .with_default_port(|_| Ok(70))
            .and_then(|host_and_port| host_and_port.to_socket_addrs())
            .map(|mut iter| iter.next().unwrap())?;

        let resource = url
            .path_segments()
            .and_then(|mut iter| iter.next().map(|item| (item, iter)))
            .and_then(|(first_arg, iter)| {
                if first_arg.len() == 0 {
                    return None;
                }
                let ty = ItemType::decode(first_arg.as_bytes()[0]);
                let rest = iter.collect::<Vec<_>>().join("/");
                Some((ty, rest))
            });

        Ok(Request { addr, resource })
    }
}

pub struct RequestCodec(Vec<String>, LinesCodec);

impl RequestCodec {
    pub fn new(ty: ItemType) -> Self {
        let inner = LinesCodec::new();
        RequestCodec(Vec::new(), inner)
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
