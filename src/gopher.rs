use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

use gtk::prelude::*;
use gtk::{
    Box as GtkBox, ButtonsType, DialogFlags, IconSize, Image, LinkButton, MessageDialog,
    MessageType, Notebook, Orientation, PackType, TextView, Widget, Window,
};
use url::Url;

use crate::tabs::BrowserExt;

#[derive(Debug)]
pub struct Request(pub SocketAddr, pub Option<(ItemType, String)>);

impl Request {
    pub fn from_url(url: &Url) -> Result<Self, ()> {
        let addr = url
            .with_default_port(|_| Ok(70))
            .and_then(|host_and_port| host_and_port.to_socket_addrs())
            .map(|mut iter| iter.next().unwrap())
            .map_err(|_| ())?;

        let path = url
            .path_segments()
            .and_then(|mut iter| iter.next().map(|item| (item, iter)))
            .and_then(|(first_arg, iter)| {
                if first_arg.len() == 0 {
                    return None;
                }
                let ty = ItemType::decode(first_arg.as_bytes()[0]);
                let rest = iter.collect::<Vec<_>>().join("/");
                Some((ty, rest))
            });

        Ok(Request(addr, path))
    }
}

#[derive(Debug)]
pub enum Response {
    Menu(Vec<MenuEntry>),
    TextFile,
    BinaryFile,
}

impl Response {
    fn menu_into_widget(
        &self,
        notebook: &Notebook,
        entries: &Vec<MenuEntry>,
    ) -> impl IsA<Widget> + Sized {
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
                    let row = GtkBox::new(Orientation::Horizontal, 15);
                    let icon = Image::new_from_icon_name("folder", IconSize::Button);
                    row.add(&icon);
                    row.set_child_packing(&icon, false, false, 20, PackType::Start);
                    let link_button = LinkButton::new_with_label(&target, Some(label.as_ref()));
                    let notebook_weak = notebook.downgrade();
                    link_button.connect_activate_link(move |_| {
                        let notebook = upgrade_weak!(notebook_weak, Inhibit(false));
                        MessageDialog::new(
                            Option::<&Window>::None,
                            DialogFlags::empty(),
                            MessageType::Info,
                            ButtonsType::Ok,
                            "hello :>",
                        )
                        .run();
                        notebook
                            .new_tab_with_url(Url::parse("gopher://sdf.org/1/users/loli").unwrap());
                        Inhibit(false)
                    });
                    row.add(&link_button);
                    container.add(&row);
                }
                SomethingElse => {}
            }
        }

        container
    }

    pub fn into_widget(&self, notebook: &Notebook) -> impl IsA<Widget> + Sized {
        match self {
            Response::Menu(entries) => self.menu_into_widget(notebook, entries),
            _ => unimplemented!("unsupported widget {:?}", self),
        }
    }
}

#[derive(Debug)]
pub enum MenuEntry {
    Information(String),
    Link(ItemType, String, String),
    SomethingElse,
}

pub fn make_request(url: Url) -> Result<Response, ()> {
    use std::io::{Read, Write};

    let request = Request::from_url(&url);

    let stream = |request: Request| -> Result<(Request, TcpStream), ()> {
        let addr = request.0;
        TcpStream::connect(&addr)
            .map(|stream| (request, stream))
            .map_err(|_| ())
    };

    let send_req =
        |(request, mut stream): (Request, TcpStream)| -> Result<(ItemType, TcpStream), ()> {
            let (to_write, item_type) = match &request.1 {
                Some((item_type, path)) => {
                    let path = path.to_string() + "\n";
                    (path, item_type.clone())
                }
                None => ("\n".to_string(), ItemType::Dir),
            };
            stream.write(to_write.as_bytes()).map_err(|err| {
                error!("Send error: {}", err);
            })?;
            Ok((item_type, stream))
        };

    let recv_res =
        |(item_type, mut stream): (ItemType, TcpStream)| -> Result<(ItemType, Vec<u8>), ()> {
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).map_err(|err| {
                error!("Recv error: {}", err);
            })?;
            Ok((item_type, buf))
        };

    let parse_output = |(item_type, buffer): (ItemType, Vec<u8>)| -> Result<Response, ()> {
        match item_type {
            ItemType::Dir => {
                let string = String::from_utf8(buffer).map_err(|err| {
                    error!("string parse error: {:?}", err);
                })?;

                let lines = string.lines();
                let mut entries = Vec::new();
                let mut current = Vec::new();

                for line in lines {
                    let line_ty = ItemType::decode(line.as_bytes()[0]);
                    let rest = &line[1..];

                    let parts = rest.split("\t").collect::<Vec<_>>();

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
                        File => entries.push(MenuEntry::Link(
                            line_ty,
                            parts[0].to_owned(),
                            parts[1].to_owned(),
                        )),
                        _ => unimplemented!("unimplemented type {:?}", line_ty),
                    }
                }
                entries.push(MenuEntry::Information(current.join("\n")));

                Ok(Response::Menu(entries))
            }
            _ => unimplemented!("unsupported type {:?}", item_type),
        }
    };

    request
        .and_then(stream)
        .and_then(send_req)
        .and_then(recv_res)
        .and_then(parse_output)
}

/// The type of a resource in a Gopher directory.
///
/// For more details, see section 3.8 of https://tools.ietf.org/html/rfc1436
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ItemType {
    /// Item is a file
    File,
    /// Item is a directory
    Dir,
    /// Item is a CSO phone-book server
    CsoServer,
    /// Error
    Error,
    /// Item is a BinHexed Macintosh file.
    BinHex,
    /// Item is DOS binary archive of some sort.
    ///
    /// Client must read until the TCP connection closes.  Beware.
    Dos,
    /// Item is a UNIX uuencoded file.
    Uuencoded,
    /// Item is an Index-Search server.
    IndexServer,
    /// Item points to a text-based telnet session.
    Telnet,
    /// Item is a binary file! Client must read until the TCP connection closes.  Beware
    Binary,
    /// Item is a redundant server
    RedundantServer,
    /// Item points to a text-based tn3270 session.
    Tn3270,
    /// Item is a GIF format graphics file.
    Gif,
    /// Item is some kind of image file.  Client decides how to display.
    Image,
    /// Item is a non-standard type
    Other(u8),
}

impl ItemType {
    pub fn decode(b: u8) -> Self {
        use ItemType::*;
        match b {
            b'0' => File,
            b'1' => Dir,
            b'2' => CsoServer,
            b'3' => Error,
            b'4' => BinHex,
            b'5' => Dos,
            b'6' => Uuencoded,
            b'7' => IndexServer,
            b'8' => Telnet,
            b'9' => Binary,
            b'+' => RedundantServer,
            b'T' => Tn3270,
            b'g' => Gif,
            b'I' => Image,
            byte => Other(byte),
        }
    }

    pub fn encode(self) -> u8 {
        use ItemType::*;
        match self {
            File => b'0',
            Dir => b'1',
            CsoServer => b'2',
            Error => b'3',
            BinHex => b'4',
            Dos => b'5',
            Uuencoded => b'6',
            IndexServer => b'7',
            Telnet => b'8',
            Binary => b'9',
            RedundantServer => b'+',
            Tn3270 => b'T',
            Gif => b'g',
            Image => b'I',
            Other(byte) => byte,
        }
    }
}
