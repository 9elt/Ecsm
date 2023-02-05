mod html;

use crate::config::ECSMConfig;
use crate::utils::logger;
use html::ECSMHtmlCompiler;

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
    html_compiler: ECSMHtmlCompiler,
    // states: States,
}

impl ECSMCompiler {
    pub fn new(config: &ECSMConfig) -> Self {
        let mut compiler = Self {
            config: config.to_owned(),
            html_compiler: ECSMHtmlCompiler::new(),
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
        self.compile_files_in(&self.config.source_path()?, "html")?;
        self.compile_files_in(&self.config.source_path()?, "css")?;
        Ok(())
    }

    pub fn compile_files_in(&mut self, dir_path: &PathBuf, target_ext: &str) -> Result<()> {
        if dir_path.exists() {
            let walker = WalkDir::new(dir_path);

            for entry in walker.into_iter().filter_map(|e| e.ok()) {
                self.compile_file(entry.path().to_path_buf(), target_ext)
                    .ok();
            }
        }
        Ok(())
    }

    pub fn compile_file(&mut self, path: PathBuf, target_ext: &str) -> Result<()> {
        match path.extension() {
            Some(ext) => match ext.to_str() {
                Some("html") => match target_ext == "html" || target_ext == "*" {
                    true => self.compile_html(path),
                    false => (),
                },
                Some("css") => match target_ext == "css" || target_ext == "*" {
                    true => self.compile_css(path),
                    false => (),
                },
                _ => (),
            },
            None => (),
        };

        Ok(())
    }

    fn check_directory(&self, path: &PathBuf) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(match path.is_dir() {
                true => path,
                false => path.parent().unwrap(),
            })?;
        }

        Ok(())
    }

    fn compile_html(&mut self, path: PathBuf) {
        logger::path_from_src(&path, &self.config);
        logger::arrow();

        self.html_compiler.reset();

        match self.html_compiler.compile(&path) {
            Ok(()) => logger::success("compiled"),
            Err(err) => logger::error(err),
        };

        logger::arrow();

        let output_path = self.get_output_path(&path);
        self.check_directory(&output_path).ok();

        match &self.html_compiler.current {
            Some(html) => {
                match html.serialize_to_file(output_path) {
                    Ok(_) => logger::success("serialized"),
                    Err(_) => logger::error("serialization failed"),
                };
            }
            None => logger::error("failed loading dom"),
        }

        logger::flush();

        // // compile global css files

        // let rel_css_dir = match &path.parent() {
        //     Some(dir) => dir.to_path_buf().join("css"),
        //     None => return (),
        // };

        // // compile related css files
        // self.compile_files_in(&rel_css_dir, "css");
    }

    fn compile_css(&mut self, path: PathBuf) {
        print!(
            "\x1b[33m\x1b[1mcompiling\x1b[0m > \x1b[1m{:?}\x1b[0m",
            path.file_name().unwrap_or(OsStr::new("missing filename"))
        );

        print!("\n");
        // print!(" \x1b[32m\x1b[1mok\x1b[0m\n");
    }

    fn get_output_path(&mut self, src_path: &PathBuf) -> PathBuf {
        PathBuf::from(
            src_path
                .to_str()
                .unwrap_or("none")
                .replace(self.config.source_dir(), self.config.output_dir()),
        )
    }

    pub fn remove_file(&mut self, path: PathBuf) -> Result<()> {
        fs::remove_file(self.get_output_path(&path))
        // remove connected states... to do...
    }

    pub fn remove_dir(&mut self, path: PathBuf) -> Result<()> {
        fs::remove_dir_all(self.get_output_path(&path))
        // remove connected states... to do...
    }
}
