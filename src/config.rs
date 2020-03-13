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
}

use std::path::PathBuf;
pub fn config_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").and_then(|h| if h.is_empty() { None } else { Some(h) });
    if let Some(home) = home {
        Some(PathBuf::from(home).join(".config"))
    } else {
        None
    }
}

pub fn get_config() -> Config {
    let config_dir = config_dir();

    let default_config = include_str!("./config.toml");

    let toml_str = if let Some(config_dir) = config_dir {
        let bar_config_dir = config_dir.join("YetAnotherBar");
        std::fs::create_dir_all(&bar_config_dir)
            .expect("Could Not Create Config Folder In .config/YetAnotherBar");
        if let Ok(file) = std::fs::read_to_string(&bar_config_dir.join("config.toml")) {
            file
        } else {
            default_config.into()
        }
    } else {
        default_config.into()
    };

    let decoded: Config = toml::from_str(&toml_str).unwrap();

    decoded
}
