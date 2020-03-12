use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Bar {
    pub name: String,
    pub monitor: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub modules_left: Vec<Module>,
    pub modules_right: Vec<Module>,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum Module {
    I3,
    Clock,
    Alsa,
    Mpris,
    Cpu,
    // Custom(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub bars: Vec<Bar>,
    // pub used_modules: Option<Vec<Module>>,
}

pub fn get_config() -> Config {
    let toml_str = include_str!("./config.toml");
    let decoded: Config = toml::from_str(toml_str).unwrap();

    decoded
}
