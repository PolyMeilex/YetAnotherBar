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
    Update(f64, i32),
    Mute,
    VolumeChange(gdk::ScrollDirection),
}

#[widget]
impl Widget for Alsa {
    fn model() -> Model {
        Model {
            alsa_mixer: alsa::Mixer::new("default", true).unwrap(),
            volume: "0%".into(),
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
                let _ = self.model.alsa_mixer.handle_events();

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
                let (min, max) = master.get_playback_volume_range();
                let volume_devider = max as f64 - min as f64;

                let add = (5.0 * volume_devider / 100.0) as i64 * mult;

                let mut volume = master
                    .get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft)
                    .unwrap();

                if volume + add > max {
                    volume = max;
                } else {
                    volume += add;
                }

                let _ = master.set_playback_volume_all(volume);
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
