use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

use crate::ModuleComponent;

pub struct ModelParam {
    pub bar_name: String,
    pub monitor_name: String,
    pub x: i32,
    pub y: i32,
    pub modules_left: Vec<ModuleComponent>,
    pub modules_right: Vec<ModuleComponent>,
}

pub struct Model {
    params: ModelParam,
}
#[derive(Msg)]
pub enum Msg {
    Quit,
    SetVisual,
}

pub struct Bar {
    gtk_window: gtk::Window,
    gtk_box: gtk::Box,
    model: Model,
}

impl Widget for Bar {
    fn init_view(&mut self) {
        for module in self.model.params.modules_left.iter() {
            self.gtk_box.pack_start(&module.widget(), false, false, 0);
        }

        for module in self.model.params.modules_right.iter() {
            self.gtk_box.pack_end(&module.widget(), false, false, 0);
        }

        self.gtk_window.show_all();
    }
    fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self {
        let gtk_window: gtk::Window = gtk::WindowBuilder::new()
            .type_(WindowType::Toplevel)
            .name(&model.params.bar_name)
            .type_hint(gdk::WindowTypeHint::Dock)
            .decorated(false)
            .default_height(35)
            .default_width(1920)
            .app_paintable(true)
            .build();

        gtk_window.move_(model.params.x, model.params.y);

        Self::set_visual(&gtk_window, None);

        let gtk_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        gtk_box.set_widget_name("main_box");
        gtk_window.add(&gtk_box);

        connect!(
            relm,
            gtk_window,
            connect_screen_changed(_, _),
            return (Some(Msg::SetVisual), ())
        );

        connect!(
            relm,
            gtk_window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Bar {
            gtk_window,
            gtk_box,
            model,
        }
    }
    type Root = Window;
    fn root(&self) -> Self::Root {
        self.gtk_window.clone()
    }
}

impl Update for Bar {
    type Msg = Msg;
    type Model = Model;
    type ModelParam = ModelParam;
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::SetVisual => Self::set_visual(&self.gtk_window, None),
        }
    }
    fn model(_: &Relm<Self>, params: ModelParam) -> Model {
        Model { params }
    }
}

impl Bar {
    fn set_visual(window: &Window, _screen: Option<&gdk::Screen>) {
        if let Some(screen) = window.get_screen() {
            if let Some(ref visual) = screen.get_rgba_visual() {
                window.set_visual(Some(visual));
            }
        }
    }
}
