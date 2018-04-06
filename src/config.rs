use std::fs::File;
use std::io::Read;

use toml;

const CONFIG_PATH: &'static str = "/etc/rs-view-tracker.conf";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub logs_dir: String,

    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_timezone")]
    pub timezone: String,
}

fn default_port() -> u16 {
    80
}

fn default_timezone() -> String {
    String::from("UTC")
}

pub fn load() -> Config {
    let mut config_file = File::open(CONFIG_PATH)
        .expect("Config file not found.");

    let mut contents = String::new();
    config_file.read_to_string(&mut contents)
        .expect("Error while reading config file.");

    toml::from_str(&contents)
        .expect("Malformed config file.")
}
