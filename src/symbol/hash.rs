use super::Symbol;
use std::hash::{Hash, Hasher};

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.subscript.hash(state);
    }
}
