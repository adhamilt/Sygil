use super::Sum;
use std::hash::{Hash, Hasher};

impl Hash for Sum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.constant.hash(state);
        for addend in self.addends.iter() {
            addend.hash(state);
        }
    }
}
