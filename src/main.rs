use gtk::prelude::*;
use relm::Component;

mod config;

mod bar;
mod clock;

mod alsa;
mod cpu;
mod i3;
mod mpris;

use bar::Bar;

pub enum ModuleComponent {
    Clock(Component<crate::clock::Clock>),
    I3(Component<crate::i3::I3>),
    Alsa(Component<crate::alsa::Alsa>),
    Mpris(Component<crate::mpris::Mpris>),
    Cpu(Component<crate::cpu::Cpu>),
}

impl ModuleComponent {
    fn widget(&self) -> gtk::Widget {
        match self {
            ModuleComponent::Clock(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::I3(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Alsa(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Mpris(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Cpu(m) => m.widget().clone().upcast::<gtk::Widget>(),
        }
    }
}

macro_rules! module_get(
    ($e:expr, $p:path) => (
        if let $p(m) = $e {
             Some(m)
        }else{
            None
        }
    )
);
macro_rules! thread_run(
    ($run:path, $module:path, $bars: expr) => (
        $run($bars
            .iter()
            .flat_map(|m| m.modules_left.iter().chain(m.modules_right.iter()))
            .filter_map(|m| module_get!(m, $module))
            .map(|m| m.stream().to_owned())
            .collect()
        );
    )
);

fn main() {
    gtk::init().unwrap();

    // Stylesheet
    {
        let stylesheet = include_bytes!("./style.css");
        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(stylesheet).unwrap();
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().unwrap(),
            &style_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // Init Bars From Config
    let bars = {
        let config = config::get_config();
        let config_bars = config.bars;

        let mut bars = Vec::new();

        for config_bar in config_bars {
            fn m(
                module: &config::Module,
                config_bar: &config::Bar,
                vec: &mut Vec<ModuleComponent>,
            ) {
                vec.push(match module {
                    config::Module::Clock => {
                        ModuleComponent::Clock(relm::init::<crate::clock::Clock>(()).unwrap())
                    }
                    config::Module::I3 => ModuleComponent::I3(
                        relm::init::<crate::i3::I3>(config_bar.monitor.clone()).unwrap(),
                    ),
                    config::Module::Alsa => {
                        ModuleComponent::Alsa(relm::init::<crate::alsa::Alsa>(()).unwrap())
                    }
                    config::Module::Mpris => {
                        ModuleComponent::Mpris(relm::init::<crate::mpris::Mpris>(()).unwrap())
                    }
                    config::Module::Cpu => {
                        ModuleComponent::Cpu(relm::init::<crate::cpu::Cpu>(()).unwrap())
                    }
                });
            }

            let mut modules_left = Vec::new();

            for module in &config_bar.modules_left {
                m(module, &config_bar, &mut modules_left);
            }

            let mut modules_right = Vec::new();

            for module in &config_bar.modules_right {
                m(module, &config_bar, &mut modules_right);
            }

            bars.push(bar::ModelParam {
                bar_name: config_bar.name,
                monitor_name: config_bar.monitor.clone(),
                x: config_bar.pos_x,
                y: config_bar.pos_y,
                modules_left,
                modules_right,
            });
        }

        bars
    };

    // Alsa Thread
    thread_run!(alsa::alsa_thread::run, ModuleComponent::Alsa, bars);
    // I3 Thread
    thread_run!(i3::i3_thread::run, ModuleComponent::I3, bars);
    // Mpris Thread
    thread_run!(mpris::mpris_thread::run, ModuleComponent::Mpris, bars);
    // Cpu Thread
    thread_run!(cpu::cpu_thread::run, ModuleComponent::Cpu, bars);

    bars.into_iter().for_each(|m| {
        let _ = relm::init::<Bar>(m).unwrap();
    });

    gtk::main();
}
