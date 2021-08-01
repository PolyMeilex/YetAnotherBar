use std::rc::Rc;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Inhibit, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

use crate::ModuleComponent;

#[derive(Clone)]
pub struct ModelParam {
    pub bar_name: String,
    pub monitor_name: String,
    pub x: i32,
    pub y: i32,
    pub modules_left: Vec<Rc<ModuleComponent>>,
    pub modules_right: Vec<Rc<ModuleComponent>>,
}

pub struct Model {
    params: ModelParam,
    app: gtk::Application,
}
#[derive(Msg)]
pub enum Msg {
    Quit,
    SetVisual,
}

pub struct Bar {
    gtk_window: ApplicationWindow,
    gtk_box: gtk::Box,
    model: Model,
}

impl Widget for Bar {
    fn init_view(&mut self) {
        for module in self.model.params.modules_left.iter() {
            self.gtk_box.pack_start(&module.widget(), false, false, 0);
        }

        for module in self.model.params.modules_right.iter().rev() {
            self.gtk_box.pack_end(&module.widget(), false, false, 0);
        }

        self.gtk_window.show_all();
    }
    fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self {
        let gtk_window: ApplicationWindow = gtk::ApplicationWindowBuilder::new()
            .application(&model.app)
            .type_(WindowType::Toplevel)
            .name(&model.params.bar_name)
            .type_hint(gdk::WindowTypeHint::Dock)
            .decorated(false)
            .default_height(35)
            .default_width(1920)
            .app_paintable(true)
            .build();

        #[cfg(feature = "wayland")]
        if gtk_layer_shell::is_supported() {
            gtk_layer_shell::init_for_window(&gtk_window);
            gtk_layer_shell::set_layer(&gtk_window, gtk_layer_shell::Layer::Top);
            gtk_layer_shell::auto_exclusive_zone_enable(&gtk_window);

            gtk_layer_shell::set_anchor(&gtk_window, gtk_layer_shell::Edge::Left, true);
            gtk_layer_shell::set_anchor(&gtk_window, gtk_layer_shell::Edge::Right, true);
            gtk_layer_shell::set_anchor(&gtk_window, gtk_layer_shell::Edge::Bottom, true);
            gtk_layer_shell::set_anchor(&gtk_window, gtk_layer_shell::Edge::Top, false);

            if let Some(display) = gdk::Display::default() {
                if let Some(monitor) = display.monitor_at_point(model.params.x, model.params.y) {
                    gtk_layer_shell::set_monitor(&gtk_window, &monitor)
                }
            }
        }

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
    type Root = ApplicationWindow;
    fn root(&self) -> Self::Root {
        self.gtk_window.clone()
    }
}

impl Update for Bar {
    type Msg = Msg;
    type Model = Model;
    type ModelParam = (ModelParam, gtk::Application);
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::SetVisual => Self::set_visual(&self.gtk_window, None),
        }
    }
    fn model(_: &Relm<Self>, params: Self::ModelParam) -> Model {
        Model {
            params: params.0,
            app: params.1,
        }
    }
}

impl Bar {
    fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
        if let Some(screen) = window.screen() {
            if let Some(ref visual) = screen.rgba_visual() {
                window.set_visual(Some(visual));
            }
        }
    }
}
