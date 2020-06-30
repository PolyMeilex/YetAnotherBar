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
        {
            let streams : Vec<_> = $bars
            .iter()
            .flat_map(|m| m.modules_left.iter().chain(m.modules_right.iter()))
            .filter_map(|m| module_get!(m, $module))
            .map(|m| m.stream().to_owned())
            .collect();

            if streams.len() > 0{
                $run(streams);
            }
        }
    )
);

fn main() {
    gtk::init().unwrap();

    let (config, stylesheet) = config::get_config();

    // Stylesheet
    {
        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(&stylesheet).unwrap();
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().unwrap(),
            &style_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let mut i3 = false;
    let mut alsa = false;
    let mut mpris = false;
    let mut cpu = false;
    // Init Bars From Config
    let bars = {
        let config_bars = config.bars;

        let mut bars = Vec::new();

        for config_bar in config_bars {
            macro_rules! match_module {
                ($module:expr, $vec:expr) => {
                    $vec.push(match $module {
                        config::Module::Clock => {
                            ModuleComponent::Clock(relm::init::<crate::clock::Clock>(()).unwrap())
                        }
                        config::Module::I3 => {
                            i3 = true;
                            ModuleComponent::I3(
                                relm::init::<crate::i3::I3>(config_bar.1.monitor.clone()).unwrap(),
                            )
                        }
                        config::Module::Alsa => {
                            alsa = true;
                            ModuleComponent::Alsa(relm::init::<crate::alsa::Alsa>(()).unwrap())
                        }
                        config::Module::Mpris => {
                            mpris = true;
                            ModuleComponent::Mpris(relm::init::<crate::mpris::Mpris>(()).unwrap())
                        }
                        config::Module::Cpu => {
                            cpu = true;
                            ModuleComponent::Cpu(relm::init::<crate::cpu::Cpu>(()).unwrap())
                        }
                    });
                };
            }

            let mut modules_left = Vec::new();

            for module in &config_bar.1.modules_left {
                match_module!(module, modules_left);
            }

            let mut modules_right = Vec::new();

            for module in &config_bar.1.modules_right {
                match_module!(module, modules_right);
            }

            bars.push(bar::ModelParam {
                bar_name: config_bar.0,
                monitor_name: config_bar.1.monitor.clone(),
                x: config_bar.1.pos_x,
                y: config_bar.1.pos_y,
                modules_left,
                modules_right,
            });
        }

        bars
    };

    // I3 Thread
    if i3 {
        thread_run!(i3::i3_thread::run, ModuleComponent::I3, bars);
    }
    // Alsa Thread
    if alsa {
        thread_run!(alsa::alsa_thread::run, ModuleComponent::Alsa, bars);
    }
    // Mpris Thread
    if mpris {
        thread_run!(mpris::mpris_thread::run, ModuleComponent::Mpris, bars);
    }
    // Cpu Thread
    if cpu {
        thread_run!(cpu::cpu_thread::run, ModuleComponent::Cpu, bars);
    }

    bars.into_iter().for_each(|m| {
        let _ = relm::init::<Bar>(m).unwrap();
    });

    gtk::main();
}
