pub fn run(streams: Vec<relm::EventStream<super::alsa::Msg>>) {
    let (_channel, sender) = relm::Channel::new(move |msg: super::alsa::Msg| {
        for s in streams.iter() {
            s.emit(msg.clone());
        }
    });
    std::thread::spawn(move || {
        fn update_event(alsa_mixer: &::alsa::Mixer) -> super::alsa::Msg {
            let master = alsa_mixer
                .find_selem(&::alsa::mixer::SelemId::new("Master", 0))
                .unwrap();
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

        // Init Event
        sender.send(update_event(&alsa_mixer)).expect("alsa send");
        loop {
            if alsa_mixer.handle_events().unwrap() == 1 {
                sender.send(update_event(&alsa_mixer)).expect("alsa send");
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
