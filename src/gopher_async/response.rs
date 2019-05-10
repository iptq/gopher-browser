use bytes::BytesMut;
use gtk::prelude::*;
use gtk::{
    Box as GtkBox, IconSize, Image, IsA, LinkButton, Notebook, Orientation, PackType, TextView,
    Widget,
};
use relm::EventStream;
use tokio::codec::{Decoder, Encoder, LinesCodec};
use url::Url;

use crate::errors::Error;
use crate::window::Msg as WindowMsg;

use super::types::ItemType;

#[derive(Debug)]
pub struct Response {
    pub url: Url,
    pub item_type: ItemType,
    pub kind: ResponseKind,
}

#[derive(Debug)]
pub enum ResponseKind {
    Menu(Vec<MenuEntry>),
    TextFile(String),
    _BinaryFile,
}

#[derive(Debug)]
pub enum MenuEntry {
    Information(String),
    Link(ItemType, String, String),
}

impl Response {
    pub fn from_buf(url: Url, item_type: ItemType, buf: Vec<u8>) -> Result<Self, Error> {
        let kind = match item_type {
            ItemType::File => {
                let string = String::from_utf8(buf).map_err(Error::from)?;
                ResponseKind::TextFile(string)
            }
            ItemType::Dir => {
                let string = String::from_utf8(buf).map_err(Error::from)?;
                let lines = string.lines();
                let mut entries = Vec::new();
                let mut current = Vec::new();

                for line in lines {
                    if line.len() == 0 {
                        continue;
                    }

                    let line_ty = ItemType::decode(line.as_bytes()[0]);
                    let rest = &line[1..];

                    let parts = rest.split("\t").collect::<Vec<_>>();
                    if parts.len() < 4 {
                        error!("Bad line from server: {:?}", line);
                        continue;
                    }

                    // join the information strings together
                    if let ItemType::Other(_) = line_ty {
                        current.push(parts[0]);
                        continue;
                    } else if !current.is_empty() {
                        entries.push(MenuEntry::Information(current.join("\n")));
                        current.clear();
                    }

                    // TODO
                    match line_ty {
                        File => {
                            let mut url = url.clone();
                            let path = parts[1].trim_start_matches("/");
                            url.set_path(&format!(
                                "{}/{}",
                                ItemType::encode(line_ty) as char,
                                path
                            ));

                            entries.push(MenuEntry::Link(
                                line_ty,
                                parts[0].to_owned(),
                                url.to_string(),
                            ))
                        }
                        _ => unimplemented!("unimplemented type {:?}", line_ty),
                    }
                }

                entries.push(MenuEntry::Information(current.join("\n")));
                ResponseKind::Menu(entries)
            }
            _ => unimplemented!("unsupported type {:?}", item_type),
        };
        Ok(Response { url, item_type, kind })
    }

    pub fn into_page(
        &self,
        notebook: &Notebook,
        stream: EventStream<WindowMsg>,
    ) -> GtkBox {
        match &self.kind {
            ResponseKind::Menu(entries) => self.menu_into_page(notebook, &entries, stream),
            ResponseKind::TextFile(contents) => self.text_into_page(contents),
            _ => unimplemented!("not supported yet"),
        }
    }

    fn text_into_page(&self, contents: impl AsRef<str>) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 0);

        let text_view = TextView::new();
        text_view.set_editable(false);
        text_view.set_cursor_visible(false);
        text_view.set_property_monospace(true);
        text_view
            .get_buffer()
            .map(|buffer| buffer.set_text(contents.as_ref()));

        container.add(&text_view);
        container
    }

    fn menu_into_page(
        &self,
        notebook: &Notebook,
        entries: &Vec<MenuEntry>,
        stream: EventStream<WindowMsg>,
    ) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 0);
        for entry in entries {
            match entry {
                MenuEntry::Information(text) => {
                    let text_view = TextView::new();
                    text_view.set_editable(false);
                    text_view.set_cursor_visible(false);
                    text_view.set_property_monospace(true);
                    text_view.get_buffer().map(|buffer| buffer.set_text(&text));
                    container.add(&text_view);
                }
                MenuEntry::Link(ty, label, target) => {
                    // TODO: don't unwrap
                    let target_url = match Url::parse(target) {
                        Ok(url) => url,
                        Err(err) => {
                            error!("Error parsing URL {}: {}", target, err);
                            continue;
                        }
                    };

                    let row = GtkBox::new(Orientation::Horizontal, 15);
                    let icon = Image::new_from_icon_name(
                        match ty {
                            ItemType::Dir => "folder",
                            _ => "text-x-generic",
                        },
                        IconSize::Button,
                    );
                    row.add(&icon);
                    row.set_child_packing(&icon, false, false, 20, PackType::Start);

                    let link_button = LinkButton::new_with_label(&target, Some(label.as_ref()));
                    let notebook_weak = notebook.downgrade();
                    let stream = stream.clone();
                    link_button.connect_activate_link(move |_| {
                        let notebook = upgrade_weak!(notebook_weak, Inhibit(false));
                        stream.emit(WindowMsg::OpenUrl(target_url.clone()));
                        Inhibit(false)
                    });
                    row.add(&link_button);
                    container.add(&row);
                }
            }
        }
        container
    }
}

pub struct ResponseCodec(ItemType);

impl ResponseCodec {
    pub fn new(item_type: ItemType) -> Self {
        ResponseCodec(item_type)
    }
}

impl Encoder for ResponseCodec {
    type Item = Response;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Decoder for ResponseCodec {
    type Item = Response;
    type Error = Error;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        info!("Decoding: {:?}", bytes);
        Ok(None)
    }
}
