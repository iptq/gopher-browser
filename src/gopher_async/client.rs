use std::net::SocketAddr;

use bytes::BytesMut;
use futures::{
    future::{self, Either},
    Future, Stream,
};
use gio::{Cancellable, SocketConnection};
use gtk::prelude::*;
use tokio::codec::{Decoder, Encoder};
use tokio::io::{read_to_end, AsyncRead};

use crate::errors::Error;

use super::request::{Request, RequestCodec};
use super::response::{Response, ResponseCodec};
use super::types::ItemType;

pub struct Client;

impl Client {
    pub fn request_async(request: Request) -> impl Future<Item = Response, Error = Error> {
        use std::io::{Read, Write};
        use tokio::net::TcpStream;

        let item_type = request.item_type;
        let mut stream = TcpStream::connect(&request.addr).map_err(Error::from);

        // send the request
        let send_request = |mut stream: TcpStream| {
            info!("Sending request to {:?}", request.addr);
            let mut request_codec = RequestCodec::new();
            let mut buf = BytesMut::new();
            request_codec.encode(request, &mut buf);
            info!("Contents: {:?}", buf);
            stream.write(&buf).map_err(Error::from).map(|_| stream)
        };

        // read the response
        let recv_response = move |mut stream: TcpStream| {
            let item_type = item_type.clone();
            match item_type {
                ItemType::Dir | ItemType::File => {
                    // read the entire body without buffering
                    Either::A(
                        read_to_end(stream, Vec::new())
                            .map(|(_, buf)| buf)
                            .map_err(Error::from)
                            .and_then(Response::from_buf),
                    )
                }
                _ => {
                    let mut response_codec = ResponseCodec::new(item_type);
                    let framed = stream.framed(response_codec);
                    // response_codec.decode(&mut buf.into()).map_err(Error::from)

                    // since there should only be response, take the next future only
                    Either::B(
                        framed
                            .into_future()
                            .map(|(head, _)| head)
                            .map_err(|(head, _)| head)
                            // TODO: don't unwrap
                            .map(|response| response.unwrap()),
                    )
                }
            }
        };

        stream.and_then(send_request).and_then(recv_response)
    }
}
