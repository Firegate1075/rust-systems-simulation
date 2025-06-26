use crate::data::expression::Expression;

pub struct Addition {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for Addition {
    
}