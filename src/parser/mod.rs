use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;

const HANDLE_STATE_ATTR: &str = "handle_state";

pub struct ECSMParser {}

impl ECSMParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_html(&self, path: &PathBuf) -> Result<()> {
        let mut html_file = File::open(path)?;
        // let html = Soup::from_reader(&html_file)?;

        let mut html_str = String::new();
        html_file.read_to_string(&mut html_str)?;

        let mut html = parse_html().one(html_str);

        Ok(())
    }
}
