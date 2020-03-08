use gtk::prelude::*;

mod bar;
mod clock;

mod alsa;
mod cpu;
mod i3;
mod mpris;

use bar::Bar;

fn main() {
    gtk::init().unwrap();

    let stylesheet = include_bytes!("./style.css");
    let style_provider = gtk::CssProvider::new();
    style_provider.load_from_data(stylesheet).unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().unwrap(),
        &style_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let monitors = vec![
        bar::ModelParam {
            bar_name: "bar-left".into(),
            monitor_name: "DP-1".into(),
            x: 0,
            y: 1000,
            components: bar::ModelComponents {
                alsa: Some(relm::init::<crate::alsa::Alsa>(()).unwrap()),
                i3: Some(relm::init::<crate::i3::I3>("DP-1".into()).unwrap()),
                mpris: Some(relm::init::<crate::mpris::Mpris>(()).unwrap()),
                cpu: Some(relm::init::<crate::cpu::Cpu>(()).unwrap()),
            },
        },
        bar::ModelParam {
            bar_name: "bar-right".into(),
            monitor_name: "HDMI-0".into(),
            x: 1920,
            y: 1000,
            components: bar::ModelComponents {
                alsa: Some(relm::init::<crate::alsa::Alsa>(()).unwrap()),
                i3: Some(relm::init::<crate::i3::I3>("HDMI-0".into()).unwrap()),
                mpris: Some(relm::init::<crate::mpris::Mpris>(()).unwrap()),
                cpu: Some(relm::init::<crate::cpu::Cpu>(()).unwrap()),
            },
        },
    ];

    // Alsa Thread
    {
        let streams = monitors
            .iter()
            .filter(|m| m.components.alsa.is_some())
            .map(|m| m.components.alsa.as_ref().unwrap().stream().to_owned())
            .collect();

        alsa::alsa_thread::run(streams);
    }

    // I3 Thread
    {
        let streams = monitors
            .iter()
            .filter(|m| m.components.i3.is_some())
            .map(|m| m.components.i3.as_ref().unwrap().stream().to_owned())
            .collect();
        i3::i3_thread::run(streams);
    }

    // Mpris Thread
    {
        let streams = monitors
            .iter()
            .filter(|m| m.components.mpris.is_some())
            .map(|m| m.components.mpris.as_ref().unwrap().stream().to_owned())
            .collect();
        mpris::mpris_thread::run(streams);
    }

    {
        let streams = monitors
            .iter()
            .filter(|m| m.components.cpu.is_some())
            .map(|m| m.components.cpu.as_ref().unwrap().stream().to_owned())
            .collect();
        cpu::cpu_thread::run(streams);
    }

    let _bar: Vec<relm::Component<crate::bar::Bar>> = monitors
        .into_iter()
        .map(|m| relm::init::<Bar>(m).unwrap())
        .collect();

    gtk::main();
}
