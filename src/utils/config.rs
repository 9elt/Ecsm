use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::File;
use std::io::{self, prelude::*, Result};

const CONFIG_NAME: &str = "ecsm.config.json";

#[derive(Serialize, Deserialize)]
pub struct ECSMConfig {
    name: String,
    source_dir: String,
}

impl ECSMConfig {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn source_dir(&self) -> &String {
        &self.source_dir
    }

    pub fn parse() -> Result<Self> {
        let path = current_dir()?.join(CONFIG_NAME);

        let mut config_file = File::open(path)?;
        let mut json_config = String::new();
        config_file.read_to_string(&mut json_config)?;

        let config: Self = serde_json::from_str(json_config.as_str())?;

        Ok(config)
    }

    pub fn new() -> Result<Self> {
        println!("\ncreate a new project | name: ");

        let mut name = String::new();
        io::stdin().read_line(&mut name)?;

        let path = current_dir()?.join(CONFIG_NAME);

        let config = Self {
            name,
            source_dir: "src".to_string(),
        };

        let json_config = serde_json::to_string(&config)?;

        std::fs::write(path, json_config)?;

        Ok(config)
    }
}
