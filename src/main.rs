use gio::prelude::*;
use gtk::prelude::*;

use relm::Component;

use std::cell::RefCell;
use std::rc::Rc;

mod config;

mod bar;
mod clock;

mod alsa;
use crate::alsa::alsa_thread::AlsaThread;
mod cpu;
use crate::cpu::cpu_thread::CpuThread;
mod i3;
use crate::i3::i3_thread::I3Thread;
mod mpris;
use crate::mpris::mpris_thread::MprisThread;

use bar::Bar;

#[derive(Clone)]
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

fn main() {
    // After update from gtk 0.8.0 to 0.8.0 there is reason for init here for some reason
    // It probably should be investigated
    gtk::init().unwrap();

    let app = gtk::Application::new(
        Some("io.github.polymeilex.yetanotherbar"),
        Default::default(),
    )
    .expect("App Init Failed");

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

    let mut i3_thread = I3Thread::new();
    let mut alsa_thread = AlsaThread::new();
    let mut mpris_thread = MprisThread::new();
    let mut cpu_thread = CpuThread::new();

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
                            let i3 = relm::init::<crate::i3::I3>((
                                config_bar.1.monitor.clone(),
                                i3_thread.sender().clone(),
                            ))
                            .unwrap();
                            i3_thread.push_stream(i3.stream().clone());

                            ModuleComponent::I3(i3)
                        }
                        config::Module::Alsa => {
                            let alsa =
                                relm::init::<crate::alsa::Alsa>(alsa_thread.sender().clone())
                                    .unwrap();
                            alsa_thread.push_stream(alsa.stream().clone());

                            ModuleComponent::Alsa(alsa)
                        }
                        config::Module::Mpris => {
                            let mpris =
                                relm::init::<crate::mpris::Mpris>(mpris_thread.sender().clone())
                                    .unwrap();
                            mpris_thread.push_stream(mpris.stream().clone());
                            ModuleComponent::Mpris(mpris)
                        }
                        config::Module::Cpu => {
                            let cpu = relm::init::<crate::cpu::Cpu>(()).unwrap();
                            cpu_thread.push_stream(cpu.stream().clone());

                            ModuleComponent::Cpu(cpu)
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
    if i3_thread.should_run() {
        i3_thread.run();
    }
    // Alsa Thread
    if alsa_thread.should_run() {
        alsa_thread.run();
    }
    // Mpris Thread
    if mpris_thread.should_run() {
        mpris_thread.run();
    }
    // Cpu Thread
    if cpu_thread.should_run() {
        cpu_thread.run();
    }

    let running = Rc::new(RefCell::new(false));

    app.connect_activate(move |app| {
        let r = *running.borrow();

        if !r {
            bars.iter().for_each(|m| {
                let _ = relm::init::<Bar>((m.clone(), app.clone())).unwrap();
            });

            running.replace(true);
        }
    });

    app.run(&std::env::args().collect::<Vec<_>>());
}
