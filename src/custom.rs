use gtk::prelude::*;
use gtk::Inhibit;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

use crate::config::CustomModule;

pub struct Model {
    config: CustomModule,
    text: String,
}

#[derive(Msg)]
pub enum Msg {
    Tick,
    Click,
}

#[widget]
impl Widget for Custom {
    fn model(config: CustomModule) -> Model {
        Model {
            config,
            text: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Click => {
                // self.model.unfolded = !self.model.unfolded;
                // self.update(Msg::Tick);
            }
            Msg::Tick => {
                if !self.model.config.exec.is_empty() {
                    let mut exec = self.model.config.exec.clone();
                    let d = std::process::Command::new(&exec.remove(0))
                        .args(exec)
                        .output();

                    if let Ok(d) = d {
                        self.model.text = String::from_utf8(d.stdout).unwrap();
                    }
                }

                // if !self.model.unfolded {
                //     self.model.time = format!("{}", time.format("%H:%M"));
                // } else {
                //     self.model.time = format!("{}", time.format("%Y-%m-%d"));
                // }
            }
        }
    }
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        self.update(Msg::Tick);
        relm::interval(relm.stream(), self.model.config.interval, || Msg::Tick);
    }

    fn init_view(&mut self) {
        self.widgets.label.set_widget_name(&self.model.config.name);
    }

    view! {
        gtk::EventBox{
            button_press_event(_,_) => (Msg::Click, Inhibit(false)),
            #[name="label"]
            gtk::Label {
                text: &self.model.text,
                // widget_name: "custom"
            },
        }
    }
}
