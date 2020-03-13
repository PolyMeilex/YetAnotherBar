use gtk::prelude::*;
use gtk::Inhibit;
use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {
    volume: String,
}

#[derive(Msg, Clone)]
pub enum Msg {
    PlayersList(String),
    PausePlay,
    Status(mpris::PlaybackStatus),
}

#[widget]
impl Widget for Mpris {
    fn model() -> Model {
        Model { volume: "0".into() }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::PlayersList(s) => self.model.volume = s,
            Msg::PausePlay => {}
            Msg::Status(status) => {
                let ctx = self.gtk_label.get_style_context();

                match status {
                    mpris::PlaybackStatus::Playing => {
                        ctx.add_class("playing");
                    }
                    _ => {
                        ctx.remove_class("playing");
                    }
                }
            }
        }
    }

    view! {
        gtk::EventBox{
            button_press_event(_,_) => (Msg::PausePlay, Inhibit(false)),

            #[name="gtk_label"]
            gtk::Label {
                text: &self.model.volume,
                widget_name: "mpris",
            },
        }
    }
}
