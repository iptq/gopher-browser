#[macro_use]
extern crate log;

mod browser;
mod gopher;
mod window;

use std::env;

use gio::prelude::*;

fn main() {
    env_logger::init();

    let application = gtk::Application::new("io.iptq.gopher-browser", Default::default())
        .expect("Initialization failed...");
    application.connect_activate(|app| {
        window::build_window(app);
    });

    application.run(&env::args().collect::<Vec<_>>());
}
