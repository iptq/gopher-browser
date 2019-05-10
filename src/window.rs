use std::sync::{Arc, Mutex};

use futures::sync::{mpsc::UnboundedSender, oneshot::Sender as OneshotSender};
use futures::Async;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Label, Notebook, Orientation, PackType,
    ScrolledWindow, SearchEntry, WindowPosition, WindowType, NONE_ADJUSTMENT,
};
use relm::{Channel, Relm, Sender, Update, Widget};
use url::Url;

use crate::errors::Error;
use crate::events::{Event, Reply};
use crate::gopher_async::{Request, Response};

pub struct Window {
    window: gtk::Window,
    notebook: gtk::Notebook,
    model: Model,
}

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

impl Update for Window {
    type Model = Model;
    type ModelParam = (OneshotSender<()>, UnboundedSender<Event>);
    type Msg = Msg;

    fn model(relm: &Relm<Self>, (stop_tx, evl_tx): Self::ModelParam) -> Model {
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
                let msg = Event::MakeRequest(request, self.model.sender.clone());
                if let Err(err) = self.model.evl_tx.send(msg) {
                    error!("Error sending request: {}", err);
                }
            }
            Msg::OpenedUrl(response) => {
                let child = GtkBox::new(Orientation::Vertical, 0);
                let stream = self.model.relm.stream().clone();
                let content = response.into_page(&self.notebook, stream);

                let search_bar = SearchEntry::new();
                child.add(&search_bar);
                child.set_child_packing(&search_bar, false, true, 0, PackType::Start);

                let content_scroll = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
                content_scroll.add(&content);
                child.add(&content_scroll);
                child.set_child_packing(&content_scroll, true, true, 0, PackType::End);

                let label = Label::new("new tab");
                let n = self.notebook.append_page(&child, Some(&label));
                self.notebook.set_tab_reorderable(&child, true);
                self.notebook.show_all();
                self.notebook.set_current_page(n);
            }
            Msg::Fail(err) => error!("error: {:?}", err),
            Msg::Quit => {
                // hack to take stop_tx
                let stop_tx = self.model.stop_tx.take();
                if let Err(err) = stop_tx.unwrap().send(()) {
                    panic!("Error sending stop: {:?}", err);
                }

                gtk::main_quit();
            }
        }
    }
}

impl Widget for Window {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = gtk::Window::new(WindowType::Toplevel);
        window.set_title("gopher-browser");
        window.set_default_size(854, 480);

        let notebook = gtk::Notebook::new();
        notebook.set_show_tabs(true);
        window.add(&notebook);

        window.show_all();
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Window {
            window,
            notebook,
            model,
        }
    }
}
