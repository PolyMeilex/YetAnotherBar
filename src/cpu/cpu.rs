use gtk::prelude::*;
use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {
    out: String,
}

#[derive(Msg, Clone)]
pub enum Msg {
    Update(String),
}

#[widget]
impl Widget for Cpu {
    fn model() -> Model {
        Model {
            out: "▁ ▁ ▁ ▁".into(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Update(s) => self.model.out = s,
        }
    }

    fn init_view(&mut self) {}

    view! {
        gtk::Label {
            text: &self.model.out,
            widget_name: "cpu"
        },
    }
}
