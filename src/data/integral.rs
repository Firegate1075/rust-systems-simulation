use crate::data::expression::Expression;

pub struct Integral {
    expression: Box<dyn Expression>,
}

impl Expression for Integral {
    
}