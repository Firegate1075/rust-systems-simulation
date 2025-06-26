use crate::data::expression::Expression;

pub struct Derivation {
    expression: Box<dyn Expression>,
}

impl Expression for Derivation {
}
