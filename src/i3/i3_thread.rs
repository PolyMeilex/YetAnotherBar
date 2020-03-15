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

fn get_workspaces_event(i3_conn: &mut i3ipc::I3Connection) -> super::i3::Msg {
    let workspaces = i3_conn
        .get_workspaces()
        .expect("i3_thread get_workspaces")
        .workspaces;

    let workspaces: Vec<I3Workspace> = workspaces
        .iter()
        .map(|ws| {
            let num = ws.num;
            let name = ws.name.clone();
            let focused = ws.focused;
            let urgent = ws.urgent;
            let output = ws.output.clone();
            I3Workspace {
                num,
                name,
                focused,
                urgent,
                output,
            }
        })
        .collect();

    super::i3::Msg::UpdateWorkspaces(workspaces)
}

pub fn run(streams: Vec<relm::EventStream<super::i3::Msg>>) {
    let (_channel, sender) = relm::Channel::new(move |msg: super::i3::Msg| {
        for s in streams.iter() {
            s.emit(msg.clone());
        }
    });

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

        let mut listener = i3ipc::I3EventListener::connect().expect("i3_thread listener");

        let subs = [i3ipc::Subscription::Mode, i3ipc::Subscription::Workspace];
        listener.subscribe(&subs).expect("i3_thread subscribe");

        for event in listener.listen() {
            use i3ipc::event::Event;
            let event = if let Ok(event) = event {
                event
            } else {
                break;
            };
            let event = match event {
                Event::WorkspaceEvent { .. } => get_workspaces_event(&mut i3_conn),
                Event::ModeEvent(info) => super::i3::Msg::UpdateMode(info.change),
                _ => continue,
            };
            sender.send(event).expect("i3_thread sennder");
        }

        // After I3 Crashed or restarted we wait 2s before trying to connect again
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
}
