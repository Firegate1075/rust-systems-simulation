use crate::data::expression::expression::Expression;
use crate::data::expression::variable::Variable;
use crate::data::signal::Signal;
use crate::data::system::System;

pub struct ConcreteSystem {
    method_var: Variable,
    definition: Box<dyn Expression>,
    input_signals: Vec<Box<dyn Signal>>,
    output_signals: Vec<Box<dyn Signal>>
}

impl System for ConcreteSystem {
    
}