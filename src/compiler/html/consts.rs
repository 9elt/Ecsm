/// #### the html class set on the `input`s states
pub const STATE_CLASS: &str = "ECSM-state";

/// #### the html input type of `boolean` states
pub const BOOLEAN_STATE_TYPE: &str = "checkbox";

/// #### the html input type of `selection` states
pub const SELECTION_STATE_TYPE: &str = "radio";

/// #### the html class set on state handlers `label`
pub const STATE_HANDLER_CLASS: &str = "ECSM-state-handler";

/// #### the state handler html attribute
/// e.g. `handle_state="test"`
pub const STATE_ATTR: &str = "handle_state";

/// #### the state handler css selector
pub const STATE_ATTR_SELECTOR: &str = "[handle_state]";

/// #### The separator between selection state `name` and `key`
/// e.g. `handle_state="test:default"`
pub const SELECTION_SEPARATOR: &str = ":";

/// #### The `key` that sets default `checked="true"` on selection states
pub const SELECTION_DEFAULT_KEY: &str = "default";

/// #### `key`s that CANNOT be used on selection states
pub const RESERVED_KEYS: [&str; 1] = ["active"];
