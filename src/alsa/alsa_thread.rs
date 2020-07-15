use std::sync::mpsc;

#[derive(Debug)]
pub enum AlsaSenderEvent {
    Mute,
    VolumeChange(f64),
}

pub struct AlsaThread {
    streams: Vec<relm::EventStream<super::alsa::Msg>>,
    tx: mpsc::Sender<AlsaSenderEvent>,
    rx: mpsc::Receiver<AlsaSenderEvent>,

    pub should_run: bool,
}

impl AlsaThread {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<AlsaSenderEvent>();

        Self {
            streams: Vec::new(),
            tx,
            rx,
            should_run: false,
        }
    }
    pub fn sender(&self) -> &mpsc::Sender<AlsaSenderEvent> {
        &self.tx
    }
    pub fn push_stream(&mut self, stream: relm::EventStream<super::alsa::Msg>) {
        self.streams.push(stream);
        self.should_run = true;
    }
    pub fn run(self) {
        let streams = self.streams;
        let (_channel, sender) = relm::Channel::new(move |msg: super::alsa::Msg| {
            for s in streams.iter() {
                s.emit(msg.clone());
            }
        });

        let rx = self.rx;
        std::thread::spawn(move || {
            fn update_event(master: &::alsa::mixer::Selem) -> super::alsa::Msg {
                let volume = master
                    .get_playback_volume(::alsa::mixer::SelemChannelId::FrontLeft)
                    .unwrap() as f64;

                let (min, max) = master.get_playback_volume_range();
                let volume_devider = max as f64 - min as f64;

                let state = master
                    .get_playback_switch(::alsa::mixer::SelemChannelId::FrontLeft)
                    .unwrap();
                super::alsa::Msg::Update((volume / volume_devider) * 100.0, state)
            }

            let alsa_mixer = ::alsa::Mixer::new("default", true).unwrap();

            let master = alsa_mixer
                .find_selem(&::alsa::mixer::SelemId::new("Master", 0))
                .unwrap();

            // Init Event
            sender.send(update_event(&master)).expect("alsa send");
            loop {
                if alsa_mixer.handle_events().unwrap() == 1 {
                    sender.send(update_event(&master)).expect("alsa send");
                }

                if let Ok(e) = rx.try_recv() {
                    match e {
                        AlsaSenderEvent::Mute => {
                            let state = master
                                .get_playback_switch(alsa::mixer::SelemChannelId::FrontLeft)
                                .unwrap();
                            if state == 0 {
                                let _ = master.set_playback_switch_all(1);
                            } else {
                                let _ = master.set_playback_switch_all(0);
                            }

                            sender.send(update_event(&master)).expect("alsa send");
                        }
                        AlsaSenderEvent::VolumeChange(v) => {
                            let (min, max) = master.get_playback_volume_range();
                            let volume_devider = max as f64 - min as f64;

                            let add = (v * volume_devider / 100.0) as i64;

                            let mut volume = master
                                .get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft)
                                .unwrap();

                            if volume + add > max {
                                volume = max;
                            } else {
                                volume += add;
                            }

                            let _ = master.set_playback_volume_all(volume);

                            sender.send(update_event(&master)).expect("alsa send");
                        }
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }
}
