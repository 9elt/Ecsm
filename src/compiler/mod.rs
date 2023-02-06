mod html;

use crate::config::ECSMConfig;
use crate::utils::logger;
use html::consts::STATE_CLASS;
use html::states::{BooleanState, SelectionState};
use html::ECSMHtmlCompiler;

use std::borrow::BorrowMut;
use std::fs;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct HtmlFile {
    pub boolean: Vec<BooleanState>,
    pub selection: Vec<SelectionState>,
    pub path: PathBuf,
    pub css_files: Vec<PathBuf>,
}

impl HtmlFile {
    pub fn from(html_compiler: &ECSMHtmlCompiler, path: PathBuf) -> Self {
        Self {
            boolean: html_compiler.boolean.to_owned(),
            selection: html_compiler.selection.to_owned(),
            path,
            css_files: html_compiler.css_files.to_owned(),
        }
    }
}

pub struct ECSMCompiler {
    config: ECSMConfig,
    html_compiler: ECSMHtmlCompiler,
    html_files: Vec<HtmlFile>, // states: States,
}

impl ECSMCompiler {
    pub fn new(config: &ECSMConfig) -> Self {
        let mut compiler = Self {
            config: config.to_owned(),
            html_compiler: ECSMHtmlCompiler::new(config.to_owned()),
            html_files: vec![],
        };

        compiler.compile_source_files().ok();

        compiler
    }

    pub fn config(&self) -> &ECSMConfig {
        &self.config
    }

    pub fn create_ecsm_css(&self) -> Result<()> {
        let path = self.config.output_path()?.join("css");
        self.check_directory(&path).ok();

        fs::write(
            path.join("ecsm.css"),
            format!(".{STATE_CLASS}{{ display: none !important }}"),
        )
        .ok();
        Ok(())
    }

    pub fn compile_source_files(&mut self) -> Result<()> {
        self.compile_files_in(&self.config.source_path()?, "html")?;
        self.create_ecsm_css().ok();
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

    fn push_html_file(&mut self, path: PathBuf) {
        match self.html_files.iter_mut().find(|f| f.path == path) {
            Some(html_file) => *html_file = HtmlFile::from(&self.html_compiler, path),
            None => self
                .html_files
                .push(HtmlFile::from(&self.html_compiler, path)),
        }
    }

    ///////////////////////////////////
    // html compiler
    ///////////////////////////////////

    fn compile_html(&mut self, path: PathBuf) {
        logger::path_from_src(&path, &self.config);
        logger::arrow();

        self.html_compiler.borrow_mut().reset();

        match self.html_compiler.compile(&path) {
            Ok(()) => logger::success("compiled"),
            Err(err) => logger::warning(err),
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

        self.push_html_file(path.to_owned());

        for css in self.html_compiler.css_files.to_owned() {
            self.compile_file(css, "css").ok();
        }
    }

    ///////////////////////////////////
    // css compiler
    ///////////////////////////////////

    fn replace_css(&self, css: &mut String, name: &String, key: &String, id: &String) {
        // spaced
        {
            let raw = format!("{}:{} ", name, key);
            let compiled = format!("#{}:checked~* ", id);

            *css = css.replace(&raw, &compiled);
        }
        // not spaced
        {
            let raw = format!("{}:{}", name, key);
            let compiled = format!("#{}:checked~", id);

            *css = css.replace(&raw, &compiled);
        }
    }

    fn compile_css(&self, path: PathBuf) {
        logger::path_from_src(&path, &self.config);
        logger::arrow();

        let mut css_file = match File::open(path.to_owned()) {
            Ok(file) => file,
            Err(_) => {
                return {
                    logger::error("failed opening file");
                    logger::flush();
                }
            }
        };

        let mut css = String::new();

        match css_file.read_to_string(&mut css) {
            Ok(_) => (),
            Err(_) => {
                return {
                    logger::error("failed reading file");
                    logger::flush();
                }
            }
        };

        let rel_html = self
            .html_files
            .iter()
            .filter(|f| f.css_files.contains(&path))
            .collect::<Vec<_>>();

        for f in rel_html {
            for boolean_state in f.boolean.iter() {
                let name = &boolean_state.name;
                let id = self.html_compiler.boolean_state_id(name);

                self.replace_css(&mut css, name, &"active".to_string(), &id);
            }
            for selection_state in f.selection.iter() {
                let name = &selection_state.name;
                for key in selection_state.keys.iter() {
                    let id = self.html_compiler.selection_state_id(name, key);

                    self.replace_css(&mut css, name, key, &id);
                }
            }
        }

        logger::success("compiled");
        logger::arrow();

        let output_path = self.get_output_path(&path);
        self.check_directory(&output_path).ok();

        match fs::write(output_path, css) {
            Ok(_) => logger::success("serialized"),
            Err(_) => logger::error("serialization failed"),
        };

        logger::flush();
    }

    ///////////////////////////////////
    // helpers
    ///////////////////////////////////

    fn get_output_path(&self, src_path: &PathBuf) -> PathBuf {
        PathBuf::from(
            src_path
                .to_str()
                .unwrap_or("none")
                .replace(self.config.source_dir(), self.config.output_dir()),
        )
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

    ///////////////////////////////////
    // remove states and files
    ///////////////////////////////////

    fn remove_html_state(&mut self, path: &PathBuf, is_dir: bool) {
        match self.html_files.iter().position(|f| match is_dir {
            true => f.path.starts_with(path),
            false => &f.path == path,
        }) {
            Some(index) => {
                self.html_files.remove(index);
            }
            None => (),
        };
    }

    pub fn remove_file(&mut self, path: PathBuf) -> Result<()> {
        fs::remove_file(self.get_output_path(&path))?;
        self.remove_html_state(&path, false);

        Ok(())
    }

    pub fn remove_dir(&mut self, path: PathBuf) -> Result<()> {
        fs::remove_dir_all(self.get_output_path(&path))?;
        self.remove_html_state(&path, true);

        Ok(())
    }
}
