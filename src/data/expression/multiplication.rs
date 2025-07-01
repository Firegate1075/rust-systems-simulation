use crate::data::expression::expression::Expression;

pub struct Multiplication {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for Multiplication {
    
}