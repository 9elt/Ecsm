use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{self, File};
use std::io::{self, prelude::*, Result};
use std::path::PathBuf;

const CONFIG_NAME: &str = "ecsm.config.json";

#[derive(Clone, Serialize, Deserialize)]
struct Directories {
    source: String,
    output: String,
    media: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ECSMConfig {
    name: String,
    dir: Directories,
}

impl ECSMConfig {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn source_dir(&self) -> &String {
        &self.dir.source
    }

    pub fn output_dir(&self) -> &String {
        &self.dir.output
    }

    pub fn media_dir(&self) -> &String {
        &self.dir.media
    }

    pub fn source_path(&self) -> Result<PathBuf> {
        Ok(current_dir()?.join(self.source_dir()))
    }

    pub fn output_path(&self) -> Result<PathBuf> {
        Ok(current_dir()?.join(self.output_dir()))
    }

    pub fn media_path(&self) -> Result<PathBuf> {
        Ok(current_dir()?.join(self.media_dir()))
    }

    pub fn check_directories(&self) -> Result<()> {
        let source_path = self.source_path()?;

        if !source_path.exists() {
            fs::create_dir(source_path)?;
        }

        let output_path = self.output_path()?;

        if !output_path.exists() {
            fs::create_dir(output_path)?;
        }

        let media_path = self.media_path()?;

        if !media_path.exists() {
            fs::create_dir(media_path)?;
        }

        Ok(())
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
        print!("\ncreate a new project | \x1b[33m\x1b[1mname\x1b[0m: ");

        io::Write::flush(&mut io::stdout()).ok();
        io::stdout().flush().ok();

        let mut name = String::new();
        io::stdin().read_line(&mut name)?;

        name = name.trim().to_string();

        let path = current_dir()?.join(CONFIG_NAME);

        let config = Self {
            name,
            dir: Directories {
                source: "src".to_string(),
                output: ".output".to_string(),
                media: "public".to_string(),
            },
        };

        let json_config = serde_json::to_string_pretty(&config)?;

        fs::write(path, json_config)?;

        Ok(config)
    }
}
