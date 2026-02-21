use crate::adapterclap::parser_in;
use crate::adapterclap::parser_matching;
use crate::adapterclap::parser_eq;
use crate::predicates::ArcPredicate;
use clap::{arg, Parser, Subcommand};
use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "k4fq", help_expected = true)]
#[command(about = "A fictional versioning CLI", long_about = None, no_binary_name = false, arg_required_else_help = true)]
pub struct CliParser {
    #[arg(long, default_value = OsStr::new(DEFAULT_CONFIG_PATH.get_or_init(|| {
                                CliParser::default_config_path().to_str().unwrap().to_string()
                            })) , help = "Location of the configuration file")]
    config: PathBuf,
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Debug, Subcommand)]
pub enum MainCommands {
    Query {
        #[command(subcommand)]
        command: QueryCommands,
    },
    #[command(
        about = "Shows the current configuration"
    )]
    ShowConfig,
}

#[derive(Debug, Subcommand)]
pub enum QueryCommands {
    Clusters {
        #[command(subcommand)]
        condition: QueryClustersCommands,
    },
    Topics {
        #[command(subcommand)]
        condition: QueryTopicsCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryClustersCommands {
    Matching {
        #[arg(allow_hyphen_values = true, value_name = "pattern", required = true, value_parser = clap::builder::ValueParser::new(parser_matching::parse), help = "Regex pattern to match against the clusters")]
        predicate: ArcPredicate<String>,
        #[command(subcommand)]
        topics: QueryClustersTopicsCommands
    },
    Eq {
        #[arg(allow_hyphen_values = true, value_name = "pattern", required = true, value_parser = clap::builder::ValueParser::new(parser_eq::parse), help = "Not empty pattern to match against the clusters")]
        predicate: ArcPredicate<String>,
        #[command(subcommand)]
        topics: QueryClustersTopicsCommands,
    },
    In {
        #[arg(allow_hyphen_values = true, value_name = "values", value_delimiter = ',', required = true, num_args = 1, value_parser = clap::builder::ValueParser::new(parser_in::parse), help = "Comma-separated cluster names")]
        predicate: ArcPredicate<String>,
        #[command(subcommand)]
        topics: QueryClustersTopicsCommands
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryClustersTopicsCommands {
    Topics {
        #[command(subcommand)]
        condition: QueryTopicsCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryTopicsCommands {
    Matching {
        #[arg(allow_hyphen_values = true, value_name = "pattern", required = true, value_parser = clap::builder::ValueParser::new(parser_matching::parse), help = "Regex pattern to match against the topics")]
        predicate: ArcPredicate<String>,
        #[command(subcommand)]
        records: QueryTopicsRecordsForCommands,
    },
    Eq {
        #[arg(allow_hyphen_values = true, value_name = "pattern", required = true, value_parser = clap::builder::ValueParser::new(parser_eq::parse), help = "Not empty pattern to match against the topics")]
        predicate: ArcPredicate<String>,
        #[command(subcommand)]
        records: QueryTopicsRecordsForCommands,
    },
    In {
        #[arg(allow_hyphen_values = true, value_name = "values", value_delimiter = ',', required = true, num_args = 1, value_parser = clap::builder::ValueParser::new(parser_in::parse), help = "Comma-separated topic names")]
        predicate: ArcPredicate<String>,
        #[command(subcommand)]
        records: QueryTopicsRecordsForCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryTopicsRecordsForCommands {
    #[command(name = "for")]
    ForClause {
        #[command(subcommand)]
        target: QueryTopicsRecordsCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryTopicsRecordsKeyCommands {
    Matching {
        #[arg(allow_hyphen_values = true, value_name = "pattern", required = true, value_parser = clap::builder::ValueParser::new(parser_matching::parse), help = "Regex pattern to match against the topics")]
        predicate: ArcPredicate<String>,
    },
    In {
        #[arg(allow_hyphen_values = true, value_name = "values", value_delimiter = ',', required = true, num_args = 1, value_parser = clap::builder::ValueParser::new(parser_in::parse), help = "Comma-separated topic names")]
        predicate: ArcPredicate<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum QueryTopicsRecordsPayloadCommands {
    Containing {
        #[arg(allow_hyphen_values = true, value_name = "pattern", required = true, value_parser = clap::builder::ValueParser::new(parser_matching::parse), help = "Regex pattern to match against the topics")]
        predicate: ArcPredicate<String>,
    },
    In {
        #[arg(allow_hyphen_values = true, value_name = "values", value_delimiter = ',', required = true, num_args = 1.., value_parser = clap::builder::ValueParser::new(parser_in::parse), help = "Comma-separated topic names")]
        predicate: ArcPredicate<String>,
    },
}


#[derive(Debug, Subcommand)]
pub enum QueryTopicsRecordsCommands {
    Key {
        #[command(subcommand)]
        condition: QueryTopicsRecordsKeyCommands,
    },
    Payload {
        #[command(subcommand)]
        condition: QueryTopicsRecordsPayloadCommands,
    },
}


impl CliParser {
    pub(crate) fn default_config_path() -> PathBuf {
        PathBuf::from(format!("{}/.k4fq/config.yaml", env::var("HOME").unwrap()))
    }
    pub fn parsed_command(&self) -> &MainCommands {
        &self.command
    }
    pub fn current_config(&self) -> &PathBuf {
        &self.config
    }
}

static DEFAULT_CONFIG_PATH: OnceLock<String> = OnceLock::new();
