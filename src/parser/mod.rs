use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

const STATE_ATTR: &str = "handle_state";

pub struct ECSMParser {}

impl ECSMParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_html(&self, path: &PathBuf) -> Result<()> {
        let mut html_file = File::open(path)?;
        let mut html_str = String::new();
        html_file.read_to_string(&mut html_str)?;

        let html = parse_html().one(html_str);

        let parsed_states = html
            .select(format!("[{}]", STATE_ATTR).as_str())
            .ok();

        if let Some(parsed_states) = parsed_states {
            let states = parsed_states
                .map(|state| {
                    let node = state.as_node();

                    let data = node.as_element().unwrap();
                    let attrs = data.attributes.to_owned().into_inner();

                    attrs
                        .map
                        .iter()
                        .filter_map(|(name, attr)| {
                            match name.local.to_string() == STATE_ATTR {
                                true => Some(attr.value.to_owned()),
                                false => None,
                            }
                        })
                        .collect::<Vec<_>>()[0]
                        .to_owned()
                })
                .collect::<Vec<_>>();
        }

        Ok(())
    }
}
