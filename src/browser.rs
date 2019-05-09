use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Label, Notebook, Orientation, PackType, ScrolledWindow, SearchEntry, Widget,
    NONE_ADJUSTMENT,
};
use url::Url;

use crate::gopher::{self, Response};

pub struct Browser {
    pub notebook: Notebook,
}

impl Browser {
    pub fn new() -> Self {
        let notebook = Notebook::new();

        Browser { notebook: notebook }
    }

    fn new_tab_with_content(&self, content: impl IsA<Widget>) {
        let child = GtkBox::new(Orientation::Vertical, 0);

        let search_bar = SearchEntry::new();
        child.add(&search_bar);
        child.set_child_packing(&search_bar, false, true, 0, PackType::Start);

        let content_scroll = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        content_scroll.add(&content);
        child.add(&content_scroll);
        child.set_child_packing(&content_scroll, true, true, 0, PackType::End);

        let label = Label::new("new tab");
        self.notebook.append_page(&child, Some(&label));
        self.notebook.set_tab_reorderable(&child, true);
    }

    pub fn new_empty_tab(&self) {
        let empty = GtkBox::new(Orientation::Vertical, 0);
        self.new_tab_with_content(empty);
    }

    pub fn new_tab_with_url(&self, url: Url) {
        use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

        gopher::make_request(url)
            .map(Response::into_widget)
            .map(|widget| {
                self.new_tab_with_content(widget);
            });
    }
}
