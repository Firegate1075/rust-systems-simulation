use crate::data::expression::expression::Expression;

pub struct Subtraction {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for Subtraction {
    
}