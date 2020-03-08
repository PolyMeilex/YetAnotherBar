use gtk::prelude::*;
use gtk::Inhibit;
use relm::{Channel, Relm, Widget};
use relm_derive::{widget, Msg};

use crate::clock::Clock;
use crate::ewmh_helper;

pub struct Model {
    monitor_name: String,
    gtk_buttons: Vec<gtk::Button>,
    cur_desktop: String,
}

#[derive(Msg)]
pub enum Msg {
    Update(ewmh_helper::EwmhEvent),
}

#[widget]
impl Widget for Ewmh {
    type ModelParam = String;

    fn model(_: &Relm<Self>, monitor_name: String) -> Model {
        Model {
            monitor_name,
            gtk_buttons: Vec::new(),
            cur_desktop: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Update(ewmh_event) => {
                self.model.cur_desktop = ewmh_event
                    .desktop_names
                    .get(ewmh_event.current_desktop as usize)
                    .unwrap()
                    .clone();

                for btn in self.model.gtk_buttons.iter() {
                    btn.destroy();
                }

                for id in 0..ewmh_event.num_desktops as usize {
                    let m = ewmh_event.desktop_monitors[id].as_ref().unwrap();
                    if m.name != self.model.monitor_name {
                        continue;
                    }
                    let btn = gtk::Button::new_with_label(&ewmh_event.desktop_names[id]);

                    if ewmh_event.current_desktop as usize == id {
                        btn.get_style_context().add_class("focused");
                    }

                    self.gtk_box.add(&btn);
                    gtk::WidgetExt::show(&btn);
                    self.model.gtk_buttons.push(btn);
                }
            }
        }
    }
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let stream = relm.stream().clone();
        let (_channel, sender) = Channel::new(move |event: ewmh_helper::EwmhEvent| {
            stream.emit(Msg::Update(event));
        });

        std::thread::spawn(|| ewmh_helper::init(sender));
    }

    fn init_view(&mut self) {}

    view! {
        #[name="gtk_box"]
        gtk::Box{
            widget_name: "ewmh",
            orientation: gtk::Orientation::Horizontal,
        }
    }
}
