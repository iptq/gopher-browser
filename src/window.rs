use std::sync::{Arc, Mutex};

use futures::sync::{mpsc::UnboundedSender, oneshot::Sender as OneshotSender};
use futures::{Async, Stream};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Notebook, WindowPosition};
use relm::{Channel, Relm, Sender, Widget};
use url::Url;

use crate::errors::Error;
use crate::events::{Event, Reply};
use crate::gopher_async::{Request, Response};

pub struct Model {
    stop_tx: Option<OneshotSender<()>>,
    evl_tx: UnboundedSender<Event>,
    channel: Channel<Reply>,
    sender: Arc<Mutex<Sender<Reply>>>,
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
        (stop_tx, evl_tx): (OneshotSender<()>, UnboundedSender<Event>),
    ) -> Model {
        let stream = relm.stream().clone();
        stream.emit(Msg::OpenUrl(
            Url::parse("gopher://sdf.org/1/users/loli").unwrap(),
        ));

        let (channel, sender) = Channel::new(move |reply| {
            match reply {
                Reply::Response(response) => stream.emit(Msg::OpenedUrl(response)),
            };
        });
        let sender = Arc::new(Mutex::new(sender));

        Model {
            stop_tx: Some(stop_tx),
            evl_tx,
            channel,
            sender,
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
                self.model
                    .evl_tx
                    .send(Event::MakeRequest(request, self.model.sender.clone()));
            }
            Msg::OpenedUrl(response) => {}
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
            resizable: false,

            gtk::Notebook {
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
