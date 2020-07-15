use gio::prelude::*;
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

    let mut mpris = false;
    let mut cpu = false;

    let mut i3_thread = i3::i3_thread::I3Thread::new();
    let mut alsa_thread = alsa::alsa_thread::AlsaThread::new();

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
                            let i = relm::init::<crate::i3::I3>((
                                config_bar.1.monitor.clone(),
                                i3_thread.sender().clone(),
                            ))
                            .unwrap();
                            i3_thread.push_stream(i.stream().clone());

                            ModuleComponent::I3(i)
                        }
                        config::Module::Alsa => {
                            let a = relm::init::<crate::alsa::Alsa>(alsa_thread.sender().clone())
                                .unwrap();
                            alsa_thread.push_stream(a.stream().clone());

                            ModuleComponent::Alsa(a)
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
    if i3_thread.should_run {
        // thread_run!(i3::i3_thread::run, ModuleComponent::I3, bars);
        i3_thread.run();
    }
    // Alsa Thread
    if alsa_thread.should_run {
        alsa_thread.run();
    }
    // Mpris Thread
    if mpris {
        thread_run!(mpris::mpris_thread::run, ModuleComponent::Mpris, bars);
    }
    // Cpu Thread
    if cpu {
        thread_run!(cpu::cpu_thread::run, ModuleComponent::Cpu, bars);
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    let running = Rc::new(RefCell::new(false));

    let r = running.clone();
    app.connect_activate(move |app| {
        let running = *r.borrow();

        if !running {
            bars.iter().for_each(|m| {
                let _ = relm::init::<Bar>((m.clone(), app.clone())).unwrap();
            });

            r.replace(true);
        }
    });

    app.run(&std::env::args().collect::<Vec<_>>());
}
