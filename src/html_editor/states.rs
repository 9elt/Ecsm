#[derive(Debug, Clone)]
pub struct BooleanState {
    pub name: String,
}

impl BooleanState {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    pub name: String,
    pub keys: Vec<String>,
}

impl SelectionState {
    pub fn new(name: String, keys: Vec<String>) -> Self {
        Self { name, keys }
    }

    pub fn add_key(&mut self, key: String) {
        if !self.keys.contains(&key) {
            self.keys.push(key)
        }
    }
}