pub struct CpuThread {
    streams: Vec<relm::EventStream<super::cpu::Msg>>,
}

impl CpuThread {
    pub fn new() -> Self {
        Self {
            streams: Vec::new(),
        }
    }
    pub fn push_stream(&mut self, stream: relm::EventStream<super::cpu::Msg>) {
        self.streams.push(stream);
    }
    pub fn should_run(&self) -> bool {
        !self.streams.is_empty()
    }
    pub fn run(self) {
        let streams = self.streams;
        let (_channel, sender) = relm::Channel::new(move |u: String| {
            for s in streams.iter() {
                s.emit(super::cpu::Msg::Update(u.clone()));
            }
        });

        std::thread::spawn(move || {
            use systemstat::{Platform, System};

            let sys = System::new();

            loop {
                match sys.cpu_load() {
                    Ok(cpu) => {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        let cpus = cpu.done().unwrap();

                        let mut load = String::new();
                        load += " ";
                        let list = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                        for cpu in cpus {
                            let cp = (cpu.user + cpu.nice + cpu.system + cpu.interrupt) * 7.0;

                            load += &list[cp as usize].to_string();
                            load += " ";
                        }
                        sender.send(load).expect("cpu_thread send message");
                    }
                    Err(x) => println!("\nCPU load: error: {}", x),
                }
            }
        });
    }
}
