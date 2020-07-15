use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct RandrMonitor {
    pub name: String,
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}
#[derive(Debug, Clone)]
pub struct I3Workspace {
    pub num: i32,
    pub name: String,
    pub focused: bool,
    pub urgent: bool,
    pub output: String,
}

#[derive(Debug)]
pub enum I3ActionEvent {
    RunCommand(String),
}

fn get_workspaces_event(i3_conn: &mut i3ipc::I3Connection) -> super::i3::Msg {
    let workspaces = i3_conn
        .get_workspaces()
        .expect("i3_thread get_workspaces")
        .workspaces;

    let workspaces: Vec<I3Workspace> = workspaces
        .into_iter()
        .map(|ws| I3Workspace {
            num: ws.num,
            name: ws.name,
            focused: ws.focused,
            urgent: ws.urgent,
            output: ws.output,
        })
        .collect();

    super::i3::Msg::UpdateWorkspaces(workspaces)
}

pub struct I3Thread {
    streams: Vec<relm::EventStream<super::i3::Msg>>,
    tx: mpsc::Sender<I3ActionEvent>,
    rx: mpsc::Receiver<I3ActionEvent>,

    pub should_run: bool,
}

impl I3Thread {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<I3ActionEvent>();

        Self {
            streams: Vec::new(),
            tx,
            rx,
            should_run: false,
        }
    }
    pub fn sender(&self) -> &mpsc::Sender<I3ActionEvent> {
        &self.tx
    }
    pub fn push_stream(&mut self, stream: relm::EventStream<super::i3::Msg>) {
        self.streams.push(stream);
        self.should_run = true;
    }
    pub fn run(self) {
        let streams = self.streams;
        let (_channel, sender) = relm::Channel::new(move |msg: super::i3::Msg| {
            for s in streams.iter() {
                s.emit(msg.clone());
            }
        });

        let rx = self.rx;
        std::thread::spawn(move || loop {
            let i3_conn = i3ipc::I3Connection::connect();

            let mut i3_conn = if let Ok(i3_conn) = i3_conn {
                i3_conn
            } else {
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            };

            // Initial Update
            sender
                .send(get_workspaces_event(&mut i3_conn))
                .expect("i3_thread sennder");

            // Sadly there is no api to get curently active binding
            // sender
            //     .send(i3_conn.get_binding_modes())
            //     .expect("i3 sennder");

            enum ThreadEvent {
                I3icpEvent(i3ipc::event::Event),
                ActionEvent(I3ActionEvent),
            }
            let local_sender = {
                let sender = sender.clone();
                let (_, local_sender) = relm::Channel::new(move |event: ThreadEvent| {
                    use i3ipc::event::Event;

                    let event = match event {
                        ThreadEvent::I3icpEvent(event) => match event {
                            Event::WorkspaceEvent { .. } => get_workspaces_event(&mut i3_conn),
                            Event::ModeEvent(info) => super::i3::Msg::UpdateMode(info.change),
                            _ => return (),
                        },
                        ThreadEvent::ActionEvent(event) => {
                            match event {
                                I3ActionEvent::RunCommand(c) => i3_conn.run_command(&c).unwrap(),
                            };
                            return ();
                        }
                    };
                    sender.send(event).expect("i3_thread sennder");
                });
                local_sender
            };

            let mut listener = i3ipc::I3EventListener::connect().expect("i3_thread listener");

            let subs = [i3ipc::Subscription::Mode, i3ipc::Subscription::Workspace];
            listener.subscribe(&subs).expect("i3_thread subscribe");

            let i3_is_running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

            {
                let local_sender = local_sender.clone();
                let i3_is_running = i3_is_running.clone();
                std::thread::spawn(move || {
                    for event in listener.listen() {
                        let event = if let Ok(event) = event {
                            event
                        } else {
                            break;
                        };
                        local_sender.send(ThreadEvent::I3icpEvent(event)).unwrap();
                    }

                    i3_is_running.swap(false, std::sync::atomic::Ordering::Relaxed);
                });
            }

            while i3_is_running.load(std::sync::atomic::Ordering::Relaxed) {
                if let Ok(e) = rx.try_recv() {
                    local_sender.send(ThreadEvent::ActionEvent(e)).unwrap();
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            // After I3 Crashed or restarted we wait 2s before trying to connect again
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
    }
}
