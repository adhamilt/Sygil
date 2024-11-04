pub mod prelude;

use crate::constant::prelude::*;
use crate::expression::Expression;
use crate::symbol::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug, Hash)]
pub enum Addend {
    Symbol(Symbol),
    Expression(Expression),
}
