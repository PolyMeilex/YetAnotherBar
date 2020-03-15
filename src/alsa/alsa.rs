use gtk::prelude::*;
use gtk::Inhibit;
use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {
    alsa_mixer: alsa::Mixer,
    volume: String,
}

#[derive(Msg, Clone)]
pub enum Msg {
    Update(i64, i32),
    Mute,
    VolumeChange(gdk::ScrollDirection),
}

#[widget]
impl Widget for Alsa {
    fn model() -> Model {
        Model {
            alsa_mixer: alsa::Mixer::new("default", true).unwrap(),
            volume: "0".into(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Update(volume, state) => {
                let p = (volume as f32 * 100.0 / 65536_f32).round();
                self.model.volume = p.to_string() + "%";

                if state == 0 {
                    self.gtk_label.get_style_context().add_class("muted");
                } else {
                    self.gtk_label.get_style_context().remove_class("muted");
                }
            }
            Msg::Mute => {
                let master = self
                    .model
                    .alsa_mixer
                    .find_selem(&alsa::mixer::SelemId::new("Master", 0))
                    .unwrap();
                let state = master
                    .get_playback_switch(alsa::mixer::SelemChannelId::FrontLeft)
                    .unwrap();
                if state == 0 {
                    let _ = master.set_playback_switch_all(1);
                } else {
                    let _ = master.set_playback_switch_all(0);
                }
            }
            Msg::VolumeChange(sd) => {
                let master = self
                    .model
                    .alsa_mixer
                    .find_selem(&alsa::mixer::SelemId::new("Master", 0))
                    .unwrap();
                let mult = match sd {
                    gdk::ScrollDirection::Up => 1,
                    gdk::ScrollDirection::Down => -1,
                    _ => return,
                };
                let add = (5.0 * 655.35) as i64 * mult;
                let volume = master
                    .get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft)
                    .unwrap();
                let _ = master.set_playback_volume_all(volume + add);
            }
        }
    }
    // fn subscriptions(&mut self, relm: &Relm<Self>) {}
    // fn init_view(&mut self) {}

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
