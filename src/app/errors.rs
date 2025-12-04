use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Errors {
    UnknownError { msg: String },
    InvalidQueryArgument { msg: String },
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self) {
            Errors::UnknownError { msg } => write!(f, "[Unknown Error] {}", msg),
            Errors::InvalidQueryArgument { msg } => { write!(f, "[Invalid query command argument] {}", msg) }
        }
    }
}

impl std::error::Error for Errors {}
