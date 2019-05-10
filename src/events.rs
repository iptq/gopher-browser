use std::sync::{Arc, Mutex};

use relm::Sender as RelmSender;

use crate::gopher_async::{Request, Response};

type Sender = Arc<Mutex<RelmSender<Reply>>>;

pub enum Event {
    MakeRequest(Request, Sender),
}

#[derive(Debug)]
pub enum Reply {
    Response(Response),
}
