use super::Product;
use std::hash::{Hash, Hasher};

impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.constant.hash(state);
        for factor in self.factors.iter() {
            factor.hash(state);
        }
    }
}
