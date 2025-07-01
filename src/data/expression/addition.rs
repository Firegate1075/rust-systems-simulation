use std::ops::Deref;
use crate::data::expression::expression::Expression;

pub struct Addition {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for Addition {
    
}

impl Addition {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Addition {
        Addition { left, right }
    }
    
    pub fn set_left(&mut self, left: Box<dyn Expression>) {
        self.left = left;
    }
    
    pub fn get_left(&self) -> &Box<dyn Expression> {
        &self.left
    }
    
    pub fn set_right(&mut self, right: Box<dyn Expression>) {
        self.right = right;
    }
    
    pub fn get_right(&self) -> &Box<dyn Expression> {
        &self.right
    }
}