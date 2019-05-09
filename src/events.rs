use crate::gopher_async::{Request, Response};

#[derive(Debug)]
pub enum Event {
    MakeRequest(Request),
}

#[derive(Debug)]
pub enum Reply {
    Response(Response),
}
