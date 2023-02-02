mod consts;
mod states;
mod utils;

use consts::*;
use states::{BooleanState, SelectionState};

use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, ExpandedName, NodeRef};
use std::{borrow::BorrowMut, fs::File, io::Read, path::PathBuf};

#[derive(Debug, Clone)]
pub struct ECSMHtmlEditor {
    pub boolean: Vec<BooleanState>,
    pub selection: Vec<SelectionState>,
    pub current: Option<NodeRef>,
}

impl ECSMHtmlEditor {
    pub fn new() -> Self {
        Self {
            boolean: vec![],
            selection: vec![],
            current: None,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn compile(&mut self, path: &PathBuf) -> Result<(), String> {
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

        let mut parser_errors = "".to_string();

        match self.borrow_mut().parse_state_hanlders(&dom) {
            Ok(_) => (),
            Err(err) => parser_errors = err,
        };

        match self.borrow_mut().insert_state_inputs(&dom) {
            Ok(_) => (),
            Err(err) => parser_errors = err,
        };

        self.current = Some(dom);

        match parser_errors == "" {
            true => Ok(()),
            false => Err(parser_errors),
        }
    }

    fn insert_state_inputs(&mut self, dom: &NodeRef) -> Result<(), String> {
        let body = match dom.select("body") {
            Ok(v) => match v.last() {
                Some(v) => v,
                None => return Err("missing body".to_string()),
            },
            Err(_) => return Err("missing body".to_string()),
        };

        for state in self.boolean.iter() {
            let input = self.create_element(
                "input",
                vec![
                    ("class", STATE_CLASS.to_string()),
                    ("id", self.boolean_state_id(state.name.as_str())),
                    ("type", BOOLEAN_STATE_TYPE.to_string()),
                ],
            );

            body.as_node().prepend(input);
        }

        for state in self.selection.iter() {
            for key in state.keys.iter() {
                let input = self.create_element(
                    "input",
                    vec![
                        ("class", STATE_CLASS.to_string()),
                        ("id", self.selection_state_id(state.name.as_str(), key)),
                        ("type", SELECTION_STATE_TYPE.to_string()),
                        (
                            "checked",
                            match key == SELECTION_DEFAULT_KEY {
                                true => "true",
                                false => "false",
                            }
                            .to_string(),
                        ),
                    ],
                );

                body.as_node().prepend(input);
            }
        }

        Ok(())
    }

    fn parse_state_hanlders(&mut self, dom: &NodeRef) -> Result<u16, String> {
        let parsed_states = match dom.select(STATE_ATTR_SELECTOR) {
            Ok(parsed) => parsed,
            Err(_) => return Ok(0),
        };

        let mut errors = "".to_string();

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
                errors = self.reserved_key_error(state_name, state_key);
                continue;
            }

            let label = self.create_element(
                "label",
                vec![
                    ("class", STATE_HANDLER_CLASS.to_string()),
                    (
                        "for",
                        match is_selection {
                            true => self.selection_state_id(state_name, state_key),
                            false => self.boolean_state_id(state_name),
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
                        prev_state.add_key(string_key);
                    }
                    None => self.selection.push(SelectionState::new(
                        state_name.to_owned(),
                        vec![state_key.to_owned()],
                    )),
                }
            } else {
                if !self.boolean.iter().any(|s| s.name == state_name) {
                    self.boolean.push(BooleanState::new(state_name.to_owned()))
                }
            }

            attr_values.push(attr_value);
        }

        if errors != "".to_string() {
            return Err(errors);
        }

        Ok(attr_values.len() as u16)
    }
}
