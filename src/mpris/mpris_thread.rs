enum MpscEvent {
    PausePlay,
}

fn find_active(finder: &mpris::PlayerFinder) -> Option<mpris::Player> {
    if let Ok(list) = finder.find_all() {
        let mut active_list: Vec<mpris::Player> = list
            .into_iter()
            .filter(|p| {
                if let Ok(mpris::PlaybackStatus::Playing) = p.get_playback_status() {
                    true
                } else {
                    false
                }
            })
            .collect();

        if active_list.len() > 0 {
            Some(active_list.remove(0))
        } else {
            finder.find_active().ok()
        }
    } else {
        None
    }
}

pub fn run(streams: Vec<relm::EventStream<super::mpris::Msg>>) {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    for s in streams.iter() {
        let tx = tx.clone();
        s.observe(move |msg| {
            if let super::mpris::Msg::PausePlay = msg {
                tx.send(MpscEvent::PausePlay).expect("mpris_thread sennder");
            }
        });
    }

    let (_channel, sender) = relm::Channel::new(move |msg: super::mpris::Msg| {
        for s in streams.iter() {
            s.emit(msg.clone());
        }
    });

    std::thread::spawn(move || {
        let finder = mpris::PlayerFinder::new().expect("Could not connect to D-Bus");
        let mut active_player: Option<mpris::Player> = finder.find_active().ok();

        loop {
            if let Some(ap) = &active_player {
                if !ap.is_running() {
                    active_player = None;
                }
            }

            if let Ok(event) = rx.try_recv() {
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
                        .send(super::mpris::Msg::Status(Some(status)))
                        .expect("mpris_thread send");
                    sender
                        .send(super::mpris::Msg::Player(Some(
                            player.identity().to_string(),
                        )))
                        .expect("mpris_thread send");
                    sender
                        .send(super::mpris::Msg::UpdateLabel)
                        .expect("mpris_thread send");

                    if let mpris::PlaybackStatus::Paused | mpris::PlaybackStatus::Stopped = status {
                        active_player = find_active(&finder);
                    }
                } else {
                    active_player = find_active(&finder);
                };
            } else {
                sender
                    .send(super::mpris::Msg::Status(None))
                    .expect("mpris_thread send");
                sender
                    .send(super::mpris::Msg::Player(None))
                    .expect("mpris_thread send");
                sender
                    .send(super::mpris::Msg::UpdateLabel)
                    .expect("mpris_thread send");

                active_player = finder.find_active().ok();
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
