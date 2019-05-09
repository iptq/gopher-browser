use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Notebook, WindowPosition};
use relm::{Relm, Widget};
use url::Url;

use crate::tabs::BrowserExt;
use crate::tabs::Tabs;

pub fn build_window(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("gopher browser");
    window.set_default_size(854, 480);
    window.set_position(WindowPosition::Center);

    let browser = Notebook::new();
    browser.new_tab_with_url(Url::parse("gopher://iptq.io").unwrap());
    browser.new_tab_with_url(
        Url::parse("gopher://sdf.org/0/users/loli/articles/modern_software_is_at_its_worst.txt")
            .unwrap(),
    );
    window.add(&browser);

    window.show_all();
}

pub struct Model {
    relm: Relm<Window>,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Window {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model { relm: relm.clone() }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            Tabs,

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
