use std::thread;
use std::time::Duration;

use gtk::prelude::*;
use gtk::Inhibit;
use relm::Channel;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

use crate::config::CustomModule;

pub struct Model {
    config: CustomModule,
    text: String,

    _channel: Channel<Msg>,
}

#[derive(Msg)]
pub enum Msg {
    Click,
    TextUpdate(String),
}

#[widget]
impl Widget for Custom {
    fn model(relm: &Relm<Self>, config: CustomModule) -> Model {
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |msg| {
            stream.emit(msg);
        });

        let c = config.clone();
        thread::spawn(move || loop {
            let mut exec = c.exec.clone();
            let d = std::process::Command::new(&exec.remove(0))
                .args(exec)
                .output();

            if let Ok(d) = d {
                let text = String::from_utf8(d.stdout).unwrap();
                sender.send(Msg::TextUpdate(text)).expect("send message");
            }

            thread::sleep(Duration::from_millis(c.interval as u64));
        });

        Model {
            config,
            text: String::new(),
            _channel: channel,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Click => {}
            Msg::TextUpdate(s) => {
                self.model.text = s;
            }
        }
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
            },
        }
    }
}
