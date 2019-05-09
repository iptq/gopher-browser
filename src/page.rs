use gtk::prelude::*;
use gtk::{ScrolledWindow, Widget as GtkWidget, NONE_ADJUSTMENT};
use relm::{Relm, Update, Widget};

pub struct Model {
    inner: GtkWidget,
    relm: Relm<Page>,
}

#[derive(Msg)]
pub enum Msg {}

pub struct Page {
    model: Model,
    root: ScrolledWindow,
}

impl Update for Page {
    type Model = Model;
    type ModelParam = GtkWidget;
    type Msg = Msg;

    fn model(relm: &Relm<Self>, inner: Self::ModelParam) -> Model {
        Model {
            inner,
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {}
}

impl Widget for Page {
    type Root = ScrolledWindow;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let root = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        root.add(&model.inner);
        Page { model, root }
    }
}
