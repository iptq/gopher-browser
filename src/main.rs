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

mod tabs;
mod gopher_async;
mod gopher;
mod window;

use std::env;

use relm::Widget;
use gio::prelude::*;

use crate::window::Window;

fn main() {
    env_logger::init();

    // let application = gtk::Application::new("io.iptq.gopher-browser", Default::default())
    //     .expect("Initialization failed...");
    // application.connect_activate(|app| {
    //     window::build_window(app);
    // });

    // application.run(&env::args().collect::<Vec<_>>());

    Window::run(());
}
