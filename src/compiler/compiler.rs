use crate::{config::ECSMConfig, html_editor::ECSMHtmlEditor};
use std::ffi::OsStr;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

// struct BooleanState {
//     name: String,
//     files: Vec<PathBuf>,
// }

// struct SelectionState {
//     name: String,
//     keys: Vec<String>,
//     files: Vec<PathBuf>,
// }

// struct States {
//     boolean: Vec<BooleanState>,
//     selection: Vec<SelectionState>,
// }

pub struct ECSMCompiler {
    config: ECSMConfig,
    html_editor: ECSMHtmlEditor,
    // states: States,
}

impl ECSMCompiler {
    pub fn new(config: &ECSMConfig) -> Self {
        let mut compiler = Self {
            config: config.to_owned(),
            html_editor: ECSMHtmlEditor::new(),
            // states: States {
            //     boolean: vec![],
            //     selection: vec![],
            // },
        };

        compiler.compile_source_files().ok();

        compiler
    }

    pub fn config(&self) -> &ECSMConfig {
        &self.config
    }

    pub fn compile_source_files(&mut self) -> Result<()> {
        self.compile_files_in(&self.config.source_path()?)
    }

    pub fn compile_files_in(&mut self, dir_path: &PathBuf) -> Result<()> {
        let walker = WalkDir::new(dir_path);

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            self.compile_file(entry.path().to_path_buf()).ok();
        }

        Ok(())
    }

    pub fn compile_file(&mut self, path: PathBuf) -> Result<()> {
        match path.extension() {
            Some(ext) => match ext.to_str() {
                Some("html") => self.compile_html(path),
                Some("css") => self.compile_css(path),
                _ => (),
            },
            None => (),
        };

        Ok(())
    }

    fn compile_html(&mut self, path: PathBuf) {
        print!(
            "\x1b[1m{:?}\x1b[0m",
            path.file_name().unwrap_or(OsStr::new("missing filename"))
        );

        print!(" >");

        self.html_editor.reset();

        match self.html_editor.compile(&path) {
            Ok(()) => print!("\x1b[32m\x1b[1m compiled\x1b[0m"),
            Err(err) => print!("\x1b[31m\x1b[1m {err}\x1b[0m"),
        };

        print!(" >");

        let output_path = self.get_output_path(path);

        match &self.html_editor.current {
            Some(html) => {
                match html.serialize_to_file(output_path) {
                    Ok(_) => print!("\x1b[32m\x1b[1m serialized\x1b[0m\n"),
                    Err(_) => print!("\x1b[31m\x1b[1m serialization failed\x1b[0m\n"),
                };
            }
            None => print!("\x1b[31m\x1b[1m failed loading dom\x1b[0m\n"),
        }
    }

    fn compile_css(&mut self, path: PathBuf) {
        print!(
            "\x1b[33m\x1b[1mcompiling\x1b[0m -> \x1b[1m{:?}\x1b[0m",
            path.file_name().unwrap_or(OsStr::new("missing filename"))
        );

        print!(" \x1b[32m\x1b[1mok\x1b[0m\n");
    }

    fn get_output_path(&mut self, src_path: PathBuf) -> PathBuf {
        PathBuf::from(
            src_path.to_str()
                .unwrap_or("none")
                .replace(self.config.source_dir(), self.config.output_dir()),
        )
    }

    pub fn remove_file(&mut self, path: PathBuf) -> Result<()> {
        fs::remove_file(self.get_output_path(path))
        // remove connected states... to do...
    }

    pub fn remove_dir(&mut self, path: PathBuf) -> Result<()> {
        fs::remove_dir_all(self.get_output_path(path))
        // remove connected states... to do...
    }
}
