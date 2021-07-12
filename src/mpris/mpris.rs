use gtk::prelude::*;
use gtk::Inhibit;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;

use super::mpris_thread;
use mpris_thread::MpscActionEvent;

pub struct Model {
    text: String,
    player: Option<String>,
    status: Option<mpris::PlaybackStatus>,
    sender: mpsc::Sender<MpscActionEvent>,
}

#[derive(Msg, Clone)]
pub enum Msg {
    UpdateLabel,
    Player(Option<String>),
    PausePlay,
    Status(Option<mpris::PlaybackStatus>),
}

#[widget]
impl Widget for Mpris {
    fn model(_: &Relm<Self>, sender: mpsc::Sender<MpscActionEvent>) -> Model {
        Model {
            text: "None".into(),
            player: None,
            status: None,
            sender,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateLabel => {
                if let (Some(player), Some(status)) = (&self.model.player, &self.model.status) {
                    self.model.text = format!("{} : {:?}", player, status);
                } else {
                    self.model.text = "None".into();
                }
            }
            Msg::Player(s) => self.model.player = s,
            Msg::PausePlay => {
                self.model.sender.send(MpscActionEvent::PausePlay).unwrap();
            }
            Msg::Status(status) => {
                self.model.status = status;

                let ctx = self.widgets.gtk_label.style_context();

                match status {
                    Some(mpris::PlaybackStatus::Playing) => {
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
                text: &self.model.text,
                widget_name: "mpris",
            },
        }
    }
}
