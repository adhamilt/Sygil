pub mod factor;
pub mod hash;
pub mod prelude;

use crate::constant::prelude::*;
use crate::expression::Expression;
use crate::symbol::prelude::*;
use factor::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Product {
    pub(super) constant: Constant,
    pub(super) factors: HashSet<Factor>,
}
