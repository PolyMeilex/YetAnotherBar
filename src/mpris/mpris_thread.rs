enum MpscEvent {
    PausePlay,
}

pub fn run(streams: Vec<relm::EventStream<super::mpris::Msg>>) {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    for s in streams.iter() {
        let tx = tx.clone();
        s.observe(move |msg| match msg {
            super::mpris::Msg::PausePlay => {
                let _ = tx.send(MpscEvent::PausePlay);
            }
            _ => {}
        });
    }

    let (_channel, sender) = relm::Channel::new(move |msg: super::mpris::Msg| {
        for s in streams.iter() {
            s.emit(msg.clone());
        }
    });

    std::thread::spawn(move || {
        // Init Event
        sender
            .send(super::mpris::Msg::PlayersList("None".into()))
            .expect("mpris_thread send");

        let finder = mpris::PlayerFinder::new().expect("Could not connect to D-Bus");
        let mut active_player: Option<mpris::Player> = finder.find_active().ok();

        loop {
            if let Some(ap) = &active_player {
                if !ap.is_running() {
                    active_player = None;
                }
            }

            if let Some(event) = rx.try_recv().ok() {
                match event {
                    MpscEvent::PausePlay => {
                        if let Some(active_player) = &active_player {
                            let _ = active_player.play_pause();
                        }
                    }
                }
            }

            if let Some(player) = &active_player {
                let status = player.get_playback_status();

                if let Ok(status) = status {
                    sender
                        .send(super::mpris::Msg::Status(status))
                        .expect("mpris_thread send");

                    sender
                        .send(super::mpris::Msg::PlayersList(
                            player.identity().to_string()
                                + &" : ".to_string()
                                + &format!("{:?}", status),
                        ))
                        .expect("mpris_thread send");
                } else {
                    active_player = finder.find_active().ok();
                };
            } else {
                sender
                    .send(super::mpris::Msg::PlayersList("None".into()))
                    .expect("mpris_thread send");

                active_player = finder.find_active().ok();
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
