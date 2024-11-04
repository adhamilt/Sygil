pub mod product;
pub mod sum;

use crate::constant::prelude::*;
use crate::symbol::prelude::*;
use product::prelude::*;
use std::sync::Arc;
use sum::prelude::*;

#[derive(Clone, Debug, Hash)]
pub enum ExpressionNode {
    Constant(Constant),
    Symbol(Symbol),
    Sum(Sum),
    Product(Product),
}

#[derive(Clone, Debug, Hash)]
pub struct Expression {
    node: Arc<ExpressionNode>,
}
