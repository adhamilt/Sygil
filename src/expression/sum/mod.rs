pub mod addend;
pub mod hash;
pub mod prelude;

use crate::constant::prelude::*;
use crate::expression::Expression;
use crate::symbol::prelude::*;
use addend::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Sum {
    pub(super) addends: HashSet<Addend>,
    pub(super) constant: Constant,
}
