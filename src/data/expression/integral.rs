use crate::data::expression::expression::Expression;

pub struct Integral {
    expression: Box<dyn Expression>,
}

impl Expression for Integral {
    
}