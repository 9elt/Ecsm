mod templates;

use crate::utils::config::ECSMConfig;
use std::fs;
use std::io::Result;

use templates::{css_template, html_template};

pub fn init_project(config: &ECSMConfig) -> Result<()> {
    match config.check_directories() {
        Ok(_) => (),
        Err(err) => panic!("failed creating directories: {:?}", err),
    }

    let css_source = config.source_path()?.join("css");

    if !css_source.exists() {
        fs::create_dir(&css_source)?;
    };

    fs::write(
        config.source_path()?.join("index.html"),
        html_template(config),
    )?;

    fs::write(&css_source.join("main.css"), css_template(config))?;

    let css_output = config.output_path()?.join("css");

    if !css_output.exists() {
        fs::create_dir(css_output)?;
    };

    Ok(())
}
