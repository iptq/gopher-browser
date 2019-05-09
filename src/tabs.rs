use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Label, Notebook, Orientation, PackType, ScrolledWindow, SearchEntry,
    Widget as GtkWidget, NONE_ADJUSTMENT,
};
use relm::{Relm, Widget};
use url::Url;

use crate::gopher_async::{Client, Error, Request, Response};

pub trait BrowserExt {
    fn new_tab_with_content(&self, content: impl IsA<GtkWidget>);
    fn new_empty_tab(&self);
    fn new_tab_with_url(&self, url: Url);
}

impl BrowserExt for Notebook {
    fn new_tab_with_content(&self, content: impl IsA<GtkWidget>) {
        let child = GtkBox::new(Orientation::Vertical, 0);

        let search_bar = SearchEntry::new();
        child.add(&search_bar);
        child.set_child_packing(&search_bar, false, true, 0, PackType::Start);

        let content_scroll = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        content_scroll.add(&content);
        child.add(&content_scroll);
        child.set_child_packing(&content_scroll, true, true, 0, PackType::End);

        let label = Label::new("new tab");
        self.append_page(&child, Some(&label));
        self.set_tab_reorderable(&child, true);
    }

    fn new_empty_tab(&self) {
        let empty = GtkBox::new(Orientation::Vertical, 0);
        self.new_tab_with_content(empty);
    }

    fn new_tab_with_url(&self, url: Url) {
        use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
        info!("loading url: {:?}", url);

        gopher::make_request(url)
            .map(|response| response.into_widget(&self))
            .map(|widget| {
                info!("done!");
                self.new_tab_with_content(widget);
            });
    }
}
