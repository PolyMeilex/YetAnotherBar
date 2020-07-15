use gtk::prelude::*;
use gtk::Inhibit;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;

use super::alsa_thread::AlsaActionEvent;

pub struct Model {
    volume: String,
    sender: mpsc::Sender<AlsaActionEvent>,
}

#[derive(Msg, Clone)]
pub enum Msg {
    Update(f64, i32),
    Mute,
    VolumeChange(gdk::ScrollDirection),
}

#[widget]
impl Widget for Alsa {
    fn model(_relm: &Relm<Self>, sender: mpsc::Sender<AlsaActionEvent>) -> Model {
        Model {
            volume: "0%".into(),
            sender,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Update(volume, state) => {
                let p = volume.round();

                self.model.volume = p.to_string() + "%";

                if state == 0 {
                    self.gtk_label.get_style_context().add_class("muted");
                } else {
                    self.gtk_label.get_style_context().remove_class("muted");
                }
            }
            Msg::Mute => {
                self.model.sender.send(AlsaActionEvent::Mute).unwrap();
            }
            Msg::VolumeChange(sd) => {
                let mult = match sd {
                    gdk::ScrollDirection::Up => 1,
                    gdk::ScrollDirection::Down => -1,
                    _ => return,
                };
                self.model
                    .sender
                    .send(AlsaActionEvent::VolumeChange(5.0 * mult as f64))
                    .unwrap();
            }
        }
    }

    view! {
        gtk::EventBox{
            events: gdk::EventMask::SCROLL_MASK,
            button_press_event(_,_) => (Msg::Mute, Inhibit(false)),
            scroll_event(_,se) => (Msg::VolumeChange(se.get_direction()),Inhibit(false)),

            #[name="gtk_label"]
            gtk::Label {
                text: &self.model.volume,
                widget_name: "alsa",
            },
        }
    }
}
