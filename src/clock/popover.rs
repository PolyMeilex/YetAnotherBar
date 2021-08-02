use gtk::prelude::*;
use gtk::EventBox;
use relm::connect;
use relm::Update;
use relm::{Relm, StreamHandle, Widget};
use relm_derive::Msg;

use super::Msg as ClockMsg;

#[derive(Msg)]
pub enum PopupMenuMsg {
    Finish,
}

pub struct PopupMenuModel {
    msg_stream: StreamHandle<ClockMsg>,
    relative_to: EventBox,
}

pub struct PopupMenu {
    model: PopupMenuModel,

    popover: gtk::Popover,
}

impl Update for PopupMenu {
    type Model = PopupMenuModel;
    type ModelParam = (StreamHandle<ClockMsg>, EventBox);
    type Msg = PopupMenuMsg;

    fn model(
        _: &Relm<Self>,
        (msg_stream, relative_to): (StreamHandle<ClockMsg>, EventBox),
    ) -> PopupMenuModel {
        PopupMenuModel {
            msg_stream,
            relative_to,
        }
    }

    fn update(&mut self, event: PopupMenuMsg) {
        match event {
            PopupMenuMsg::Finish => {
                self.popover.popdown();
                self.model.msg_stream.emit(ClockMsg::PopoverDone);
            }
        }
    }
}

impl Widget for PopupMenu {
    type Root = gtk::Popover;

    fn init_view(&mut self) {
        self.popover.popup();
    }

    fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self {
        let popover = gtk::PopoverBuilder::new()
            .position(gtk::PositionType::Top)
            .constrain_to(gtk::PopoverConstraint::None)
            .relative_to(&model.relative_to)
            .build();
        popover.popdown();

        let vbox = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .margin(10)
            .build();
        popover.set_child(Some(&vbox));

        let calendar = gtk::CalendarBuilder::new()
            .width_request(300)
            .height_request(150)
            .build();
        vbox.pack_start(&calendar, true, true, 0);
        vbox.show_all();

        connect!(
            relm,
            popover,
            connect_closed(_),
            return (Some(PopupMenuMsg::Finish), ())
        );

        Self { model, popover }
    }

    fn root(&self) -> Self::Root {
        self.popover.clone()
    }
}
