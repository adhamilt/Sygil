use super::Constant;
use std::hash::{Hash, Hasher};

impl Hash for Constant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
