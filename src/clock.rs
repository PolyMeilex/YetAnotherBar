use gtk::prelude::*;
use gtk::Inhibit;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

pub struct Model {
    unfolded: bool,
    time: String,
}

#[derive(Msg)]
pub enum Msg {
    Tick,
    Click,
}

#[widget]
impl Widget for Clock {
    fn model() -> Model {
        let time = chrono::Local::now();
        Model {
            unfolded: false,
            time: format!("{}", time.format("%H:%M")),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Click => {
                self.model.unfolded = !self.model.unfolded;
                self.update(Msg::Tick);
            }
            Msg::Tick => {
                let time = chrono::Local::now();

                if !self.model.unfolded {
                    self.model.time = format!("{}", time.format("%H:%M"));
                } else {
                    self.model.time = format!("{}", time.format("%Y-%m-%d"));
                }
            }
        }
    }
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        relm::interval(relm.stream(), 1000, || Msg::Tick);
    }

    fn init_view(&mut self) {}

    view! {
        gtk::EventBox{
            button_press_event(_,_) => (Msg::Click, Inhibit(false)),
            gtk::Label {
                text: &self.model.time,
                widget_name: "clock"
            },
        }
    }
}
