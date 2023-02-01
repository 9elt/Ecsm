use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, NodeRef};
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use html5ever::{interface::QualName, local_name, namespace_url, ns};
use kuchiki::{Attribute, ExpandedName};

const STATE_ATTR: &str = "handle_state";
const STATE_ATTR_SELECTOR: &str = "[handle_state]";

const SELECTION_SEPARATOR: &str = ":";
const SELECTION_DEFAULT_KEY: &str = "default";
const RESERVED_KEYS: [&str; 1] = ["active"];

#[derive(Debug, Clone)]
pub struct BooleanState {
    name: String,
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    name: String,
    keys: Vec<String>,
}

pub struct ECSMParser {
    boolean: Vec<BooleanState>,
    selection: Vec<SelectionState>,
}

impl ECSMParser {
    pub fn new() -> Self {
        Self {
            boolean: vec![],
            selection: vec![],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn parse_html(&mut self, path: &PathBuf) -> Result<(), String> {
        let mut html_file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("failed opening".to_string()),
        };

        let mut html_str = String::new();
        match html_file.read_to_string(&mut html_str) {
            Ok(_) => (),
            Err(_) => return Err("read failed".to_string()),
        };

        let dom = parse_html().one(html_str);
        match self.borrow_mut().parse_state(&dom) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }

        // println!("\nstates -> {:?}", states);
        // println!("\nstates -> {:?}", self.selection);
        // println!("\nhtml ->\n{}\n", dom.to_string());
    }

    fn boolean_state_id(&self, state_name: &str) -> String {
        format!("ECSM-boolean-ID-{state_name}")
    }

    fn selection_state_id(&self, state_name: &str, state_key: &str) -> String {
        format!("ECSM-selection-ID-{state_name}-KEY-{state_key}")
    }

    fn parse_state(&mut self, dom: &NodeRef) -> Result<u16, String> {
        let parsed_states = match dom.select(STATE_ATTR_SELECTOR) {
            Ok(parsed) => parsed,
            Err(_) => return Err("no states".to_string()),
        };

        let mut errors= "".to_string();

        let mut attr_values = vec![];

        for state in parsed_states {
            let target = ExpandedName::new("", STATE_ATTR);

            let mut clean = state.attributes.clone().into_inner();
            let attr_value = match clean.map.borrow_mut().remove(&target) {
                Some(v) => v.value.to_owned(),
                None => continue,
            };

            let split = attr_value.split(SELECTION_SEPARATOR).collect::<Vec<_>>();

            let is_selection = split.len() > 1;

            let state_name = split[0];

            let state_key = match is_selection {
                true => split[1],
                false => "",
            };

            if RESERVED_KEYS.contains(&state_key) {
                errors = format!("reserved key \"{state_key}\" used by \"{state_name}\"");
            }

            let _is_default = state_key == SELECTION_DEFAULT_KEY;

            let label = NodeRef::new_element(
                QualName::new(None, ns!(html), local_name!("label")),
                vec![
                    (
                        ExpandedName::new("", "class"),
                        Attribute {
                            prefix: None,
                            value: "ECSM-state-handler".to_owned(),
                        },
                    ),
                    (
                        ExpandedName::new("", "for"),
                        Attribute {
                            prefix: None,
                            value: match is_selection {
                                true => self.selection_state_id(state_name, state_key),
                                false => self.boolean_state_id(state_name),
                            },
                        },
                    ),
                ],
            );

            *state.attributes.borrow_mut() = clean;

            state.as_node().insert_after(label.to_owned());
            label.append(state.as_node().to_owned());

            // continue if duplicate state

            if attr_values.contains(&attr_value) {
                continue;
            }

            // push states

            if is_selection {
                let prev = self.selection.iter_mut().find(|s| s.name == state_name);

                match prev {
                    Some(prev_state) => {
                        let string_key = state_key.to_string();
                        if !prev_state.keys.contains(&string_key) {
                            prev_state.keys.push(string_key);
                        }
                    }
                    None => self.selection.push(SelectionState {
                        name: state_name.to_owned(),
                        keys: vec![state_key.to_owned()],
                    }),
                }
            } else {
                if !self.boolean.iter().any(|s| s.name == state_name) {
                    self.boolean.push(BooleanState {
                        name: state_name.to_owned(),
                    })
                }
            }

            attr_values.push(attr_value);
        }

        if errors != "".to_string() {
            return Err(errors);
        }

        match attr_values.len() {
            0 => Err("no states".to_string()),
            _ => Ok(attr_values.len() as u16),
        }
    }
}
