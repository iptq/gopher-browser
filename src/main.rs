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
mod window;

use std::env;
use std::sync::Arc;
use std::thread;

use futures::sync::{mpsc, oneshot};
use gio::prelude::*;
use relm::Widget;
use tokio::runtime::Runtime;

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

    let (evl_tx, evl_rx) = mpsc::unbounded::<()>();

    // let (to_thread, from_thread) = mpsc::channel();
    // let (to_gui, from_gui) = mpsc::channel();

    thread::spawn(move || {
        Window::run((stop_tx, evl_tx));
    });

    runtime.block_on(stop_rx);
}
