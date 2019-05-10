#[macro_use]
extern crate log;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

#[macro_use]
mod utils;

mod errors;
mod events;
mod gopher_async;
mod window;

use std::env;
use std::sync::Arc;
use std::thread;

use futures::sync::{mpsc, oneshot};
use futures::{Future, Stream};
use gio::prelude::*;
use relm::{Channel, Widget};
use tokio::runtime::Runtime;

use crate::errors::Error;
use crate::events::{Event, Reply};
use crate::window::Window;

fn main() {
    env_logger::init();

    let mut runtime = Runtime::new().expect("failed to create runtime");
    let (stop_tx, stop_rx) = oneshot::channel::<()>();

    let (evl_tx, evl_rx) = mpsc::unbounded::<Event>();
    let (gui_tx, gui_rx) = mpsc::unbounded::<Reply>();

    let gui_tx = Arc::new(gui_tx);

    let evl = evl_rx
        .map_err(|_| Error::ChannelRecv)
        .for_each(move |event| match event {
            Event::MakeRequest(request, sender) => {
                use crate::gopher_async::Client;
                let gui_tx = gui_tx.clone();
                Client::request_async(request).and_then(move |response| {
                    sender
                        .lock()
                        .unwrap()
                        .send(Reply::Response(response))
                        .map_err(Error::from)
                })
            }
        })
        .map_err(|err| {
            error!("Error: {:?}", err);
        });
    runtime.spawn(evl);

    thread::spawn(move || {
        Window::run((stop_tx, evl_tx)).expect("error");
    });

    runtime.block_on(stop_rx);
    info!("Exiting.");
}
