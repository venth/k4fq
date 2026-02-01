use predicates::BoxPredicate;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Command {
    Skip { cause: String },
    ShowConfig { config: PathBuf },
    Query { config: PathBuf, cluster_matcher: BoxPredicate<String>, topic_matcher: BoxPredicate<String>, payload_matcher: BoxPredicate<String>},
}

impl Command {
    pub fn skip_because_of<T: ToString>(value: T) -> Self {
        Command::Skip { cause: value.to_string() }
    }
}

#[derive(Debug)]
pub struct Topic {
    name: String,
}

impl From<String> for Topic {
    fn from(name: String) -> Self {
        Self { name }
    }
}

pub trait CommandParser: Send + Sync {
    fn parse(&self, args: &Vec<String>) -> Command;
}
