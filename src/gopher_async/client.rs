use std::net::SocketAddr;

use bytes::BytesMut;
use futures::Future;
use tokio::codec::{Decoder, Encoder};
use tokio::io::AsyncRead;

use super::errors::Error;
use super::request::{Request, RequestCodec};
use super::response::{Response, ResponseCodec};

pub struct Client;

impl Client {
    pub fn request_async(
        addr: &SocketAddr,
        request: Request,
    ) -> impl Future<Item = Option<Response>, Error = Error> {
        use std::io::{Read, Write};
        use tokio::net::TcpStream;

        let mut stream = TcpStream::connect(addr).map_err(Error::from);

        // send the request
        let send_request = |mut stream: TcpStream| {
            let mut request_codec = RequestCodec::new();
            let mut buf = BytesMut::new();
            request_codec.encode(request, &mut buf);
            stream.write(&buf).map_err(Error::from).map(|_| stream)
        };

        // read the response
        let recv_response = |mut stream: TcpStream| {
            let mut response_codec = ResponseCodec::new();
            // let framed = stream.framed(response_codec);
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf);
            response_codec.decode(&mut buf.into()).map_err(Error::from)
        };

        stream.and_then(send_request).and_then(recv_response).boxed()
    }
}
