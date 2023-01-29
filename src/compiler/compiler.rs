use crate::{config::ECSMConfig, parser::ECSMParser};
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

struct Files {
    html: Vec<PathBuf>,
    css: Vec<PathBuf>,
}

struct BooleanState {
    name: String,
}

struct SelectionState {
    name: String,
    keys: Vec<String>,
}

struct States {
    boolean: Vec<BooleanState>,
    selection: Vec<SelectionState>,
}

pub struct ECSMCompiler {
    config: ECSMConfig,
    parser: ECSMParser,
    files: Files,
    states: States,
}

impl ECSMCompiler {
    pub fn new(config: &ECSMConfig) -> Self {
        let mut compiler = Self {
            config: config.to_owned(),
            parser: ECSMParser::new(),
            files: Files {
                html: vec![],
                css: vec![],
            },
            states: States {
                boolean: vec![],
                selection: vec![],
            },
        };

        match compiler.get_source_files() {
            Ok(_) => (),
            Err(err) => println!("error finding source files {:?}", err),
        }

        compiler
    }

    pub fn config(&self) -> &ECSMConfig {
        &self.config
    }

    pub fn get_source_files(&mut self) -> Result<()> {
        self.get_files(&self.config.source_path()?)
    }

    pub fn get_files(&mut self, dir_path: &PathBuf) -> Result<()> {
        let walker = WalkDir::new(dir_path);

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                continue;
            }

            match entry.path().extension() {
                Some(ext) => match ext.to_str() {
                    Some("html") => self.files.html.push(entry.path().to_path_buf()),
                    Some("css") => self.files.css.push(entry.path().to_path_buf()),
                    _ => continue,
                },
                None => continue,
            }
        }

        Ok(())
    }
}
