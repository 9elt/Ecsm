use std::path::PathBuf;

use crate::{config::ECSMConfig, parser::ECSMParser};

struct Files {
    html: Vec<PathBuf>,
    css: Vec<PathBuf>,
}

pub struct ECSMCompiler {
    config: ECSMConfig,
    parser: ECSMParser,
    files: Files,
}

impl ECSMCompiler {
    pub fn new(config: &ECSMConfig) -> Self {
        Self {
            config: config.to_owned(),
            parser: ECSMParser::new(),
            files: Files {
                html: vec![],
                css: vec![],
            },
        }
    }

    pub fn config(&self) -> &ECSMConfig {
        &self.config
    }
}
