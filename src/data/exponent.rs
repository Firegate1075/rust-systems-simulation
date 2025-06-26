use crate::data::expression::Expression;

pub struct Exponent {
    base: Box<dyn Expression>,
    exponent: Box<dyn Expression>,
}

impl Expression for Exponent {
    
}