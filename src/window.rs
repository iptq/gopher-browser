use std::sync::Arc;

use futures::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot::Sender as OneshotSender,
};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Notebook, WindowPosition};
use relm::{Relm, Widget};
use url::Url;

use crate::events::{Event, Reply};
use crate::gopher_async::{Error, Request, Response};

// pub fn build_window(app: &Application) {
//     let window = ApplicationWindow::new(app);
//     window.set_title("gopher browser");
//     window.set_default_size(854, 480);
//     window.set_position(WindowPosition::Center);

//     let browser = Notebook::new();
//     browser.new_tab_with_url(Url::parse("gopher://iptq.io").unwrap());
//     browser.new_tab_with_url(
//         Url::parse("gopher://sdf.org/0/users/loli/articles/modern_software_is_at_its_worst.txt")
//             .unwrap(),
//     );
//     window.add(&browser);

//     window.show_all();
// }

pub struct Model {
    stop_tx: Option<OneshotSender<()>>,
    evl_tx: UnboundedSender<Event>,
    gui_rx: UnboundedReceiver<Reply>,
    relm: Relm<Window>,
}

#[derive(Msg)]
pub enum Msg {
    OpenUrl(Url),
    OpenedUrl(Response),
    Fail(Error),
    Quit,
}

#[widget]
impl Widget for Window {
    fn model(
        relm: &Relm<Self>,
        (stop_tx, evl_tx, gui_rx): (
            OneshotSender<()>,
            UnboundedSender<Event>,
            UnboundedReceiver<Reply>,
        ),
    ) -> Model {
        let stream = relm.stream();
        stream.emit(Msg::OpenUrl(
            Url::parse("gopher://sdf.org/1/users/loli").unwrap(),
        ));

        Model {
            stop_tx: Some(stop_tx),
            evl_tx,
            gui_rx,
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::OpenUrl(url) => {
                info!("Opening URL {:?}", url);
                // TODO: don't unwrap
                let request = Request::from_url(url).unwrap();
                info!("Request {:?}", request);

                // spawn the event on the event loop
                self.model.evl_tx.send(Event::MakeRequest(request));
            }
            Msg::OpenedUrl(resp) => {}
            Msg::Fail(err) => error!("error: {:?}", err),
            Msg::Quit => {
                // hack to take stop_tx
                let stop_tx = self.model.stop_tx.take();
                stop_tx.unwrap().send(());

                gtk::main_quit();
            }
        }
    }

    view! {
        gtk::Window {
            title: "gopher-browser",
            property_default_width: 854,
            property_default_height: 480,

            gtk::Notebook {
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
