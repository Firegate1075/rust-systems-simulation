use crate::data::signal::Signal;

pub struct Simulation {
    start_signal: Box<dyn Signal>,
    end_signal: Box<dyn Signal>
}