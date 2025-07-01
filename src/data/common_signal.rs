use crate::data::expression::expression::Expression;
use crate::data::signal::Signal;
use crate::data::system::System;

pub struct CommonSignal {
    left_system: Option<Box<dyn System>>,
    right_system: Option<Box<dyn System>>,
    value: Box<dyn Expression>
}

impl Signal for CommonSignal {
    
}