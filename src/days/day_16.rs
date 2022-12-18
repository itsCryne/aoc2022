use std::fmt::Display;

pub fn a(_input: &str) -> Box<dyn Display> {
   Box::new(0)
}

pub fn b(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}
