use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, WindowPosition};
use url::Url;

use crate::browser::Browser;

pub fn build_window(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("gopher browser");
    window.set_default_size(854, 480);
    window.set_position(WindowPosition::Center);

    let browser = Browser::new();
    browser.new_tab_with_url(Url::parse("gopher://sdf.org:70/1/users/loli").unwrap());
    window.add(&browser.notebook);

    window.show_all();
}
