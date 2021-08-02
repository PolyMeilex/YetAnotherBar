use gdk::EventButton;
use gtk::prelude::*;
use gtk::Inhibit;
use relm::Component;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

mod popover;
use popover::{PopupMenu, PopupMenuMsg};

pub struct Model {
    unfolded: bool,
    time: String,

    relm: Relm<Clock>,

    popup_menu: Option<Component<PopupMenu>>,
}

#[derive(Msg)]
pub enum Msg {
    Tick,
    Click(EventButton),
    PopoverDone,
}

#[widget]
impl Widget for Clock {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let time = chrono::Local::now();
        Model {
            unfolded: false,
            time: format!("{}", time.format("%H:%M")),

            relm: relm.clone(),
            popup_menu: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Click(btn_event) => {
                if btn_event.button() == 3 && self.model.popup_menu.is_none() {
                    self.model.popup_menu = Some(relm::create_component::<PopupMenu>((
                        self.model.relm.stream().clone(),
                        self.widgets.event_box.clone(),
                    )));
                } else if btn_event.button() == 1 {
                    self.model.unfolded = !self.model.unfolded;
                    self.update(Msg::Tick);
                }
            }
            Msg::Tick => {
                let time = chrono::Local::now();

                if !self.model.unfolded {
                    self.model.time = format!("{}", time.format("%H:%M"));
                } else {
                    self.model.time = format!("{}", time.format("%Y-%m-%d"));
                }
            }
            Msg::PopoverDone => self.model.popup_menu = None,
        }
    }
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        relm::interval(relm.stream(), 1000, || Msg::Tick);
    }

    view! {
        #[name="event_box"]
        gtk::EventBox{
            button_press_event(_,event) => (Msg::Click(event.clone()), Inhibit(false)),
            gtk::Label {
                text: &self.model.time,
                widget_name: "clock"
            },
        }
    }
}
