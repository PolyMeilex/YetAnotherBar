use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{connect, Component, Relm, Update, Widget};
use relm_derive::Msg;

pub struct ModelParam {
    pub bar_name: String,
    pub monitor_name: String,
    pub x: i32,
    pub y: i32,
    pub components: ModelComponents,
}

pub struct ModelComponents {
    pub alsa: Option<Component<crate::alsa::Alsa>>,
    pub i3: Option<Component<crate::i3::I3>>,
    pub mpris: Option<Component<crate::mpris::Mpris>>,
    pub cpu: Option<Component<crate::cpu::Cpu>>,
}

pub struct Model {
    params: ModelParam,
    // i3: Component<crate::i3::I3>,
    clock: Component<crate::clock::Clock>,
    
    // alsa: Component<crate::alsa::Alsa>,
    // mpris: Component<crate::mpris::Mpris>,
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
        if let Some(i3) = self.model.params.components.i3.as_ref() {
            self.gtk_box.pack_start(i3.widget(), false, false, 0);
        }
        self.gtk_box
            .pack_end(self.model.clock.widget(), false, false, 0);

        if let Some(alsa) = self.model.params.components.alsa.as_ref() {
            self.gtk_box.pack_end(alsa.widget(), false, false, 0);
        }
        if let Some(mpris) = self.model.params.components.mpris.as_ref() {
            self.gtk_box.pack_end(mpris.widget(), false, false, 0);
        }

        if let Some(cpu) = self.model.params.components.cpu.as_ref() {
            self.gtk_box.pack_end(cpu.widget(), false, false, 0);
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
        Model {
            params,
            clock: relm::init::<crate::clock::Clock>(()).unwrap(),
        }
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
