use super::consts::*;
use super::ECSMHtmlEditor;

use html5ever::{interface::QualName, namespace_url, ns, LocalName};
use kuchiki::{Attribute, ExpandedName, NodeRef};

impl ECSMHtmlEditor {
    pub fn boolean_state_id(&self, state_name: &str) -> String {
        format!("ECSM-boolean-ID-{state_name}")
    }

    pub fn selection_state_id(&self, state_name: &str, state_key: &str) -> String {
        format!("ECSM-selection-ID-{state_name}-KEY-{state_key}")
    }

    pub fn reserved_key_error(&self, state_name: &str, state_key: &str) -> String {
        format!("reserved keyword \"{state_key}\" ~ {STATE_ATTR}=\"{state_name}:{state_key}\"")
    }

    pub fn create_element(&self, tag_name: &str, attrs: Vec<(&str, String)>) -> NodeRef {
        NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from(tag_name)),
            attrs
                .iter()
                .map(|attr| {
                    (
                        ExpandedName::new("", attr.0.to_owned()),
                        Attribute {
                            prefix: None,
                            value: attr.1.to_owned(),
                        },
                    )
                })
                .collect::<Vec<_>>(),
        )
    }
}
