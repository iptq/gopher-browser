#[macro_use]
extern crate log;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

#[macro_use]
mod utils;

// mod gopher;
mod gopher_async;
mod page;
// mod tabs;
mod events;
mod window;

use std::env;
use std::sync::Arc;
use std::thread;

use futures::sync::{mpsc, oneshot};
use futures::{Future, Stream};
use gio::prelude::*;
use relm::Widget;
use tokio::runtime::Runtime;

use crate::events::{Event, Reply};
use crate::window::Window;

fn main() {
    env_logger::init();

    // let application = gtk::Application::new("io.iptq.gopher-browser", Default::default())
    //     .expect("Initialization failed...");
    // application.connect_activate(|app| {
    //     window::build_window(app);
    // });

    // application.run(&env::args().collect::<Vec<_>>());

    let mut runtime = Runtime::new().expect("failed to create runtime");
    let (stop_tx, stop_rx) = oneshot::channel::<()>();

    let (evl_tx, evl_rx) = mpsc::unbounded::<Event>();
    let (gui_tx, gui_rx) = mpsc::unbounded::<Reply>();

    // let (to_thread, from_thread) = mpsc::channel();
    // let (to_gui, from_gui) = mpsc::channel();

    thread::spawn(move || {
        Window::run((stop_tx, evl_tx, gui_rx));
    });

    runtime.spawn(
        evl_rx
            .map(move |event| match event {
                Event::MakeRequest(request) => {
                    use crate::gopher_async::Client;
                    Client::request_async(request)
                        .map(|response| gui_tx.send(Reply::Response(response)));
                }
            })
            .collect()
            .map(|_| ()),
    );
    runtime.block_on(stop_rx);
}
