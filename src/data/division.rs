use crate::data::expression::Expression;

pub struct Division {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for Division {

}