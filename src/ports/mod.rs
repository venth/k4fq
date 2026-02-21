mod dynamic_config;

use crate::app::Errors;
pub use crate::ports::dynamic_config::DynamicConfig;
use crate::predicates::ArcPredicate;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Command {
    Skip { cause: String },
    ShowConfig { config: PathBuf },
    Query { config: PathBuf, cluster_matcher: ArcPredicate<String>, topic_matcher: ArcPredicate<String>, payload_matcher: ArcPredicate<String>},
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

pub trait ConfigurationSource: Send + Sync {
    fn load(&self, config: &PathBuf) -> Result<DynamicConfig, Errors>;
}

pub trait Reporter: Send {
    fn stage(&self, stage_name: &str);
    fn info(&self, msg: &str);
    fn inc(&self, delta: u64);
    fn finish(self: Box<Self>);
}

pub trait ReporterFactory: Send + Sync {
    fn create_unchained_reporter(&self, name: &str) -> Box<dyn Reporter>;
    fn create_chained_reporter(&self, name: &str, len: u64) -> Box<dyn Reporter>;
}
