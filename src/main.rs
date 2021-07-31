use gio::{prelude::*, ApplicationFlags};
use gtk::prelude::*;

use relm::Component;

use std::cell::RefCell;
use std::rc::Rc;

mod config;

mod bar;
mod clock;
mod custom;

mod alsa;
use crate::alsa::alsa_thread::AlsaThread;
mod cpu;
use crate::cpu::cpu_thread::CpuThread;
mod i3;
use crate::i3::i3_thread::I3Thread;
mod mpris;
use crate::mpris::mpris_thread::MprisThread;

use bar::Bar;

pub enum ModuleComponent {
    Clock(Component<crate::clock::Clock>),
    I3(Component<crate::i3::I3>),
    Alsa(Component<crate::alsa::Alsa>),
    Mpris(Component<crate::mpris::Mpris>),
    Cpu(Component<crate::cpu::Cpu>),
    Custom(Component<crate::custom::Custom>),
}

impl ModuleComponent {
    fn widget(&self) -> gtk::Widget {
        match self {
            ModuleComponent::Clock(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::I3(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Alsa(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Mpris(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Cpu(m) => m.widget().clone().upcast::<gtk::Widget>(),
            ModuleComponent::Custom(m) => m.widget().clone().upcast::<gtk::Widget>(),
        }
    }
}

pub struct Threads {
    i3: I3Thread,
    alsa: AlsaThread,
    mpris: MprisThread,
    cpu: CpuThread,
}

fn main() {
    let app = gtk::Application::new(
        Some("io.github.polymeilex.yetanotherbar"),
        ApplicationFlags::HANDLES_COMMAND_LINE,
    );

    let bars: Rc<RefCell<Vec<bar::ModelParam>>> = Default::default();

    app.connect_command_line(move |app, cli| {
        if !cli.is_remote() {
            let (config, stylesheet) = config::get_config();

            let mut bars = bars.borrow_mut();

            // Stylesheet
            {
                let style_provider = gtk::CssProvider::new();
                style_provider.load_from_data(&stylesheet).unwrap();
                gtk::StyleContext::add_provider_for_screen(
                    &gdk::Screen::default().unwrap(),
                    &style_provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }

            let mut threads = Threads {
                i3: I3Thread::new(),
                alsa: AlsaThread::new(),
                mpris: MprisThread::new(),
                cpu: CpuThread::new(),
            };

            // Init Bars From Config
            for config_bar in config.bars {
                fn match_module(
                    monitor: String,
                    module: config::Module,
                    threads: &mut Threads,
                    list: &mut Vec<Rc<ModuleComponent>>,
                ) {
                    list.push(Rc::new(match module {
                        config::Module::Clock => {
                            ModuleComponent::Clock(relm::init::<crate::clock::Clock>(()).unwrap())
                        }
                        config::Module::I3 => {
                            let i3 =
                                relm::init::<crate::i3::I3>((monitor, threads.i3.sender().clone()))
                                    .unwrap();
                            threads.i3.push_stream(i3.stream().clone());

                            ModuleComponent::I3(i3)
                        }
                        config::Module::Alsa => {
                            let alsa =
                                relm::init::<crate::alsa::Alsa>(threads.alsa.sender().clone())
                                    .unwrap();
                            threads.alsa.push_stream(alsa.stream().clone());

                            ModuleComponent::Alsa(alsa)
                        }
                        config::Module::Mpris => {
                            let mpris =
                                relm::init::<crate::mpris::Mpris>(threads.mpris.sender().clone())
                                    .unwrap();
                            threads.mpris.push_stream(mpris.stream().clone());
                            ModuleComponent::Mpris(mpris)
                        }
                        config::Module::Cpu => {
                            let cpu = relm::init::<crate::cpu::Cpu>(()).unwrap();
                            threads.cpu.push_stream(cpu.stream().clone());

                            ModuleComponent::Cpu(cpu)
                        }
                        config::Module::Custom(config) => ModuleComponent::Custom(
                            relm::init::<crate::custom::Custom>(config).unwrap(),
                        ),
                    }));
                }

                let mut param = bar::ModelParam {
                    bar_name: config_bar.0,
                    monitor_name: config_bar.1.monitor.clone(),
                    x: config_bar.1.pos_x,
                    y: config_bar.1.pos_y,
                    modules_left: Vec::new(),
                    modules_right: Vec::new(),
                };

                for module in config_bar.1.modules_left {
                    match_module(
                        config_bar.1.monitor.clone(),
                        module,
                        &mut threads,
                        &mut param.modules_left,
                    );
                }

                for module in config_bar.1.modules_right.clone() {
                    match_module(
                        config_bar.1.monitor.clone(),
                        module,
                        &mut threads,
                        &mut param.modules_right,
                    );
                }

                bars.push(param);
            }

            // I3 Thread
            if threads.i3.should_run() {
                threads.i3.run();
            }
            // Alsa Thread
            if threads.alsa.should_run() {
                threads.alsa.run();
            }
            // Mpris Thread
            if threads.mpris.should_run() {
                threads.mpris.run();
            }
            // Cpu Thread
            if threads.cpu.should_run() {
                threads.cpu.run();
            }

            bars.iter().for_each(|m| {
                let _ = relm::init::<Bar>((m.clone(), app.clone())).unwrap();
            });
        }

        0
    });

    app.run();
}
