use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ApError {
    error: String
}

impl ApError {
    pub fn new(error: String) -> ApError {
        ApError { error }
    }
}

impl Debug for ApError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error)
    }
}

impl Display for ApError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error)
    }
}

impl Error for ApError {

}