use std::fmt::{Debug, Display, Formatter};

pub struct PredicateError {
    msg: String
}

impl Debug for PredicateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PredicateError: {}", self.msg)
    }
}

impl Display for PredicateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for PredicateError {}

impl From<String> for PredicateError {
    fn from(value: String) -> Self {
        PredicateError { msg: value }
    }
}
