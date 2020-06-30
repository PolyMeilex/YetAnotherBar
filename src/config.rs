use serde_derive::Deserialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Bar {
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

// #[derive(Clone, Debug, Deserialize)]
// pub struct DetailedDep {
//     name: String,
// }

// #[derive(Clone, Debug, Deserialize)]
// #[serde(untagged)]
// pub enum Dep {
//     Simple(String),
//     Detailed(DetailedDep),
// }

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub bars: BTreeMap<String, Bar>,
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

pub fn get_config() -> (Config, Vec<u8>) {
    let config_dir = config_dir();

    let default_config = include_str!("./config.toml");
    let default_style = include_bytes!("./style.css");

    let (toml_str, style_str) = if let Some(config_dir) = config_dir {
        let bar_config_dir = config_dir.join("YetAnotherBar");
        let _ = std::fs::create_dir_all(&bar_config_dir);
        let toml_str =
            if let Ok(file) = std::fs::read_to_string(&bar_config_dir.join("config.toml")) {
                file
            } else {
                default_config.into()
            };
        let style_str = if let Ok(file) = std::fs::read(&bar_config_dir.join("style.css")) {
            file
        } else {
            default_style.to_vec()
        };
        (toml_str, style_str)
    } else {
        (default_config.into(), default_style.to_vec())
    };

    let decoded: Config = toml::from_str(&toml_str).unwrap();

    (decoded, style_str)
}
