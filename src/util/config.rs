use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub storage: StorageType,
    pub mongodb_connection_string: Option<String>,
    pub master_key: String,
}

#[derive(Clone, Deserialize)]
pub enum StorageType {
    Mongo,
    Json,
}

impl Config {
    pub fn from_file() -> Self {
        let mut file = File::open("config.json").expect("Config file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Error reading config file");
        serde_json::from_str(&contents).expect("Error parsing config file")
    }
}
