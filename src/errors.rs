use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

use futures::sync::mpsc::SendError;

use crate::events::Reply;

#[derive(Debug)]
pub enum Error {
    ChannelRecv,
    SendReply(SendError<Reply>),
    String(FromUtf8Error),
    IO(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<SendError<Reply>> for Error {
    fn from(err: SendError<Reply>) -> Self {
        Error::SendReply(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::String(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelRecv => write!(f, "ChannelRecv"),
            Error::IO(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl StdError for Error {}
