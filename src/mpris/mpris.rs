use gdk::prelude::*;
use gtk::prelude::*;
use gtk::Inhibit;

use relm::{Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;

use super::mpris_thread;
use mpris_thread::MpscActionEvent;

pub struct Model {
    text: String,
    player: Option<String>,
    status: Option<mpris::PlaybackStatus>,
    sender: mpsc::Sender<MpscActionEvent>,

    test: Vec<gtk::Window>,
    album_buf: Option<gdk_pixbuf::Pixbuf>,

    self_stream: relm::StreamHandle<Msg>,
}

#[derive(Msg, Clone)]
pub enum Msg {
    UpdateLabel,
    Player(Option<String>),
    PausePlay,
    Status(Option<mpris::PlaybackStatus>),
    ArtImg(Vec<u8>),
    ShowPopup,

    ButtonPressEvent(u32),
}

#[widget]
impl Widget for Mpris {
    fn model(relm: &Relm<Self>, sender: mpsc::Sender<MpscActionEvent>) -> Model {
        let self_stream = relm.stream().clone();
        Model {
            text: "None".into(),
            player: None,
            status: None,
            sender,

            test: Vec::new(),
            album_buf: None,

            self_stream,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateLabel => {
                if let (Some(player), Some(status)) = (&self.model.player, &self.model.status) {
                    self.model.text = format!("{} : {:?}", player, status);
                } else {
                    self.model.text = "None".into();
                }
            }
            Msg::Player(s) => self.model.player = s,
            Msg::PausePlay => {
                self.model.sender.send(MpscActionEvent::PausePlay).unwrap();
            }
            Msg::Status(status) => {
                self.model.status = status;

                let ctx = self.widgets.gtk_label.style_context();

                match status {
                    Some(mpris::PlaybackStatus::Playing) => {
                        ctx.add_class("playing");
                    }
                    _ => {
                        ctx.remove_class("playing");
                    }
                }
            }
            Msg::ArtImg(data) => {
                use gdk_pixbuf::prelude::*;
                use gdk_pixbuf::PixbufLoader;

                let loader = PixbufLoader::new();
                loader.write(&data).unwrap();
                loader.close().unwrap();
                let full_buf = loader.pixbuf().unwrap();
                self.model.album_buf = Some(full_buf);

                // let w = full_buf.get_width();
                // let h = full_buf.get_height();

                // let hh = h / 20;

                // let buf = full_buf
                //     .scale_simple(w / hh, h / hh, gdk_pixbuf::InterpType::Bilinear)
                //     .unwrap();
                //
                // let display = window.get_display();
                // let seat = display.get_default_seat().unwrap();
                // let status = seat.grab(&window, gdk::SeatCapabilities::ALL, true, None, None, None);
                //
                //

                // for win in std::mem::replace(&mut self.model.test, Vec::new()) {
                //     win.close()
                // }

                // self.gtk_label
                //     .connect_query_tooltip(move |_, _, _, _, tool| {
                //         let img = gtk::ImageBuilder::new()
                //             .pixbuf(&full_buf)
                //             .has_tooltip(true)
                //             .build();

                //         tool.set_custom(Some(&img));
                //         true
                //     });
            }

            Msg::ShowPopup => {
                if mpris_thread::POPUP_OPEN.load(std::sync::atomic::Ordering::Acquire) {
                    return;
                } else {
                    mpris_thread::POPUP_OPEN.store(true, std::sync::atomic::Ordering::Release);
                }

                let win = gtk::WindowBuilder::new()
                    .events(gdk::EventMask::FOCUS_CHANGE_MASK)
                    .type_(gtk::WindowType::Popup)
                    .window_position(gtk::WindowPosition::Mouse)
                    .build();

                let gtk_box = gtk::BoxBuilder::new()
                    .orientation(gtk::Orientation::Vertical)
                    .parent(&win)
                    .build();

                gtk::ImageBuilder::new()
                    .pixbuf(&self.model.album_buf.as_ref().unwrap())
                    .parent(&gtk_box)
                    .build();

                let btn = gtk::ButtonBuilder::new()
                    .label("Play/Pause")
                    .parent(&gtk_box)
                    .build();

                let sender = self.model.self_stream.clone();
                btn.connect_clicked(move |_| {
                    sender.emit(Msg::PausePlay);
                });

                win.show_all();

                {
                    let (x, y, w, h) = win.window().unwrap().geometry();
                    win.move_(x, y - 35);
                }

                let display = win.display();
                let seat = display.default_seat().unwrap();
                let gdk_window = win.window().unwrap();

                let status = seat.grab(
                    &gdk_window,
                    gdk::SeatCapabilities::ALL_POINTING,
                    true,
                    None,
                    None,
                    None,
                );

                println!("{:?}", status);

                win.connect_focus_in_event(|win, dir| {
                    println!("in");
                    gtk::Inhibit(false)
                });

                win.connect_focus_out_event(|win, dir| {
                    println!("out");
                    // win.close();
                    Inhibit(false)
                });

                win.connect_button_press_event(move |win, ev| {
                    println!("press");

                    let (mx, my) = ev.position();
                    let (x, y, w, h) = win.window().unwrap().geometry();
                    let (x, y, w, h) = (x as f64, y as f64, w as f64, h as f64);

                    let is_in = mx > 0.0 && mx < w && my > 0.0 && my < h;

                    println!("x:{},y:{}", mx, my);
                    println!("x:{},y:{},w:{},h{}", x, y, w, h);

                    println!("{}", is_in);

                    if !is_in {
                        println!("click inside");
                        seat.ungrab();
                        win.close();
                        mpris_thread::POPUP_OPEN.store(false, std::sync::atomic::Ordering::Release);
                    }

                    //
                    //
                    Inhibit(false)
                });

                self.model.test.push(win);
            }

            Msg::ButtonPressEvent(id) => {
                if id == 1 {
                    self.update(Msg::PausePlay);
                } else if id == 3 {
                    self.update(Msg::ShowPopup);
                }
            }
        }
    }

    view! {
        gtk::EventBox{
            button_press_event(_,ev) => (Msg::ButtonPressEvent(ev.button()), Inhibit(false)),

            #[name="gtk_label"]
            gtk::Label {
                widget_name: "mpris",
                text: &self.model.text,
                has_tooltip: true
            },
        }
    }
}
