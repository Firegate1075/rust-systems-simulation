use crate::data::expression::expression::Expression;

pub struct Derivation {
    expression: Box<dyn Expression>,
}

impl Expression for Derivation {
    
}

impl Derivation {
    pub fn new(expression: Box<dyn Expression>) -> Derivation {
        Derivation { expression }
    }
    
    pub fn set_expression(&mut self, expression: Box<dyn Expression>) {
        self.expression = expression;
    }
    
    pub fn get_expression(&self) -> &Box<dyn Expression> {
        &self.expression
    }
}
