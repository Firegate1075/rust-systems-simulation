use crate::data::expression::expression::Expression;

pub struct Exponent {
    base: Box<dyn Expression>,
    exponent: Box<dyn Expression>,
}

impl Expression for Exponent {
    
}

impl Exponent {
    pub fn new(base: Box<dyn Expression>, exponent: Box<dyn Expression>) -> Exponent {
        Exponent { base, exponent }
    }
    
    pub fn set_base(&mut self, base: Box<dyn Expression>) {
        self.base = base;
    }
    
    pub fn get_base(&self) -> &Box<dyn Expression> {
        &self.base
    }
    
    pub fn set_exponent(&mut self, exponent: Box<dyn Expression>) {
        self.exponent = exponent;
    }
    
    pub fn get_exponent(&self) -> &Box<dyn Expression> {
        &self.exponent
    }
}