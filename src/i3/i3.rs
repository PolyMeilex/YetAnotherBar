use gtk::prelude::*;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;

use super::i3_thread;
use i3_thread::I3SenderEvent;

pub struct Model {
    monitor_name: String,
    gtk_buttons: Vec<gtk::Button>,
    gtk_label: gtk::Label,
    sender: mpsc::Sender<I3SenderEvent>,
}

#[derive(Msg, Clone)]
pub enum Msg {
    UpdateWorkspaces(Vec<i3_thread::I3Workspace>),
    UpdateMode(String),
}

#[widget]
impl Widget for I3 {
    type ModelParam = String;

    fn model(
        _: &Relm<Self>,
        (monitor_name, sender): (String, mpsc::Sender<I3SenderEvent>),
    ) -> Model {
        Model {
            monitor_name,
            gtk_buttons: Vec::new(),
            gtk_label: gtk::LabelBuilder::new().name("mode").build(),
            sender,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateWorkspaces(workspaces) => {
                for btn in self.model.gtk_buttons.iter() {
                    btn.destroy();
                }

                for ws in workspaces.iter() {
                    if ws.output != self.model.monitor_name {
                        continue;
                    }
                    let btn = gtk::Button::new_with_label(&ws.name);

                    if ws.focused {
                        btn.get_style_context().add_class("focused");
                    }
                    if ws.urgent {
                        btn.get_style_context().add_class("urgent");
                    }

                    let name = ws.name.clone();

                    let sender = self.model.sender.clone();
                    btn.connect_clicked(move |_| {
                        sender
                            .send(I3SenderEvent::RunCommand(format!("workspace {}", name)))
                            .unwrap();
                    });

                    self.gtk_box.pack_start(&btn, false, false, 0);
                    gtk::WidgetExt::show(&btn);
                    self.model.gtk_buttons.push(btn);
                }
            }
            Msg::UpdateMode(mode) => {
                if mode != "default" {
                    self.model.gtk_label.set_text(&mode);
                } else {
                    self.model.gtk_label.set_text("");
                }
            }
        }
    }
    fn init_view(&mut self) {
        self.gtk_box
            .pack_end(&self.model.gtk_label, false, false, 5);
    }

    view! {
        #[name="gtk_box"]
        gtk::Box{
            widget_name: "i3",
            orientation: gtk::Orientation::Horizontal,
        }
    }
}
