use crate::data::signal::Signal;
use crate::data::simulation::Simulation;
use crate::data::system::System;

pub struct RecursiveSystem {
    sub_simulation: Simulation,
    input_signals: Vec<Box<dyn Signal>>,
    output_signals: Vec<Box<dyn Signal>>
}

impl System for RecursiveSystem {
    
}