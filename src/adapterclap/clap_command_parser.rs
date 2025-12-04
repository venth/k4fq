use crate::ports;
use crate::ports::Command;
use clap::{arg, value_parser, ArgAction};
use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::OnceLock;
use crate::adapterclap::parser_in;

struct ClapCommandParser {}

pub fn new() -> impl ports::CommandParser {
    ClapCommandParser {}
}

impl ClapCommandParser {
    pub(crate) fn default_config_path(&self) -> PathBuf {
        PathBuf::from(format!("{}/.k4fq/config.yaml", env::var("HOME").unwrap()))
    }

    fn record_payload_command() -> clap::Command {
        clap::Command::new("payload")
            .about("Queries records with the matching payload in the selected topics")
            .subcommand(
                clap::Command::new("containing")
                    .about("payload contains content matching the regex pattern")
                    .arg(
                        arg!(<PATTERN> "Regex pattern to match against the payload's content")
                            .required(true)
                    )
            )
    }

    fn record_key_command() -> clap::Command {
        clap::Command::new("key")
            .about("Queries records with the matching key in the selected topics")
            .subcommand(clap::Command::new("in"))
            .subcommand(clap::Command::new("matching"))
            .subcommand(clap::Command::new("eq"))
    }

    fn topics_command() -> clap::Command {
        clap::Command::new("topics")
            .about("Selects topics using criteria")
            .subcommand(clap::Command::new("in")
                .about("Select topics in the given collection of comma-separated values")
                .arg(arg!(<VALUES> "Comma-separated topic names")
                    .required(true)
                    .value_delimiter(',')
                    .num_args(1..)
                    .value_parser(clap::builder::ValueParser::new(parser_in::parse))
                    .action(ArgAction::Append))
                .subcommand(
                    clap::Command::new("for")
                        .about("Queries records in the matching topics for the given criteria")
                        .subcommand(Self::record_key_command())
                        .subcommand(Self::record_payload_command()))
            )
            .subcommand(
                clap::Command::new("matching")
                    .about("Select topics matching the regex pattern")
                    .arg(arg!(<PATTERN> "Regex pattern to match against the topics")
                        .required(true))
                    .subcommand(
                        clap::Command::new("for")
                            .about("Queries records in the matching topics for the given criteria")
                            .subcommand(Self::record_key_command())
                            .subcommand(Self::record_payload_command())
                    )
            )
            .subcommand(clap::Command::new("eq")
                .about("Select topics equal to the given value")
                .arg(arg!(<VALUE> "Topic name").required(true))
                .subcommand(
                    clap::Command::new("for")
                        .about("Queries records in the matching topics for the given criteria")
                        .subcommand(Self::record_key_command())
                        .subcommand(Self::record_payload_command()))
            )
    }
}

static DEFAULT_CONFIG_PATH: OnceLock<String> = OnceLock::new();

impl ports::CommandParser for ClapCommandParser {
    fn parse(&self, args: &Vec<String>) -> Command {
        clap::command!()
            .no_binary_name(true)
            .arg_required_else_help(true)
            .arg(
                arg!(--config <FILE >"Location of the configuration file")
                    .value_parser(value_parser!(PathBuf))
                    .required(false)
                    .default_value(OsStr::new(DEFAULT_CONFIG_PATH.get_or_init(|| {
                        self.default_config_path().to_str().unwrap().to_string()
                    }))),
            )
            .subcommand(
                clap::Command::new("query")
                    .subcommand(
                        clap::Command::new("clusters")
                            .subcommand(clap::Command::new("in"))
                            .subcommand(clap::Command::new("matching"))
                            .subcommand(clap::Command::new("eq")),
                    )
                    .subcommand(
                        Self::topics_command(),
                    )
            )
            .try_get_matches_from(args)
            .map(|m| match m.subcommand() {
                None => Command::skip_because_of("help or version chosen"),
                Some((cmd, matching)) => {
                    todo!()
                }
            })
            .unwrap_or_else(|e| {
                eprint!("{}", e);
                return Command::skip_because_of(format!("Unrecognized command or flag. The details: {}", e));
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{ClapCommandParser, DEFAULT_CONFIG_PATH};
    use crate::ports::{Command, CommandParser};
    use proptest::prelude::{prop, Strategy};
    use proptest::{prop_compose, prop_oneof, proptest};
    use std::path::PathBuf;

    fn parser() -> impl CommandParser {
        ClapCommandParser {}
    }

    #[test]
    fn parses_help() {
        let a = args_of("--help");
        let result = parser().parse(&a);
        assert_matches!(result, Command::Skip { .. })
    }

    #[test]
    fn parses_version() {
        let a = args_of("--version");
        let result = parser().parse(&a);

        assert_matches!(result, Command::Skip { .. })
    }

    #[test]
    fn parses_even_on_unknown_flag_or_command() {
        let a = args_of("--bla bu bu");
        let result = parser().parse(&a);

        assert_matches!(result, Command::Skip { .. });
    }

    #[test]
    fn returns_default_config_value_if_no_config_flag_is_passed() {
        let a = args_of("--bla bu bu");
        let result = parser().parse(&a);

        let default_config_path = PathBuf::from(DEFAULT_CONFIG_PATH.get().unwrap());
        assert_matches!(result, Command::Skip { .. });
    }

    proptest! {
        #[test]
        fn returns_query_by_payload_command_if_query_by_payload_command_is_passed(query_arg in arb_query_arg()) {
            let result = parser().parse(&args_of(query_arg.as_str()));

            let default_config_path = PathBuf::from(DEFAULT_CONFIG_PATH.get().unwrap());
            assert_matches!(result, Command::Query { config, .. } if config == default_config_path );
        }
    }

    prop_compose! {
        fn arb_cluster_name()(s in "[a-zA-Z0-9_-]+") -> String { s.to_string() }
    }

    prop_compose! {
        fn arb_clusters_criteria_arg()(criteria in prop::sample::select(vec!["matching", "in", "eq"])) -> String {
            criteria.to_string()
        }
    }

    prop_compose! {
        fn arb_clusters_arg() (cluster_criteria in arb_clusters_criteria_arg(), cluster_name in arb_cluster_name()) -> String {
            format!("clusters {} {}", cluster_criteria, cluster_name)
        }
    }

    prop_compose! {
        fn arb_topic_name()(s in "[a-zA-Z0-9_-]+") -> String { s.to_string() }
    }

    prop_compose! {
        fn arb_matching_arg()(pattern in arb_regex()) -> String { format!("matching {}", pattern.to_string()) }
    }

    prop_compose! {
        fn arb_topics_eq_arg()(name in arb_topic_name()) -> String { format!("eq {}", name.to_string()) }
    }

    prop_compose! {
        fn arb_topics_in_arg()(topics in prop::collection::vec(arb_topic_name(), 1..5)) -> String { format!("in {}", topics.join(",")) }
    }

    prop_compose! {
        fn arb_topics_matching_arg()(regex in arb_regex()) -> String { format!("matching {}", regex) }
    }

    fn arb_topics_criteria_arg(arb_matching: impl Strategy<Value=String>, arb_in: impl Strategy<Value=String>, arb_eq: impl Strategy<Value=String>)
        -> impl Strategy<Value=String> {
        prop_oneof![arb_matching, arb_in, arb_eq]
    }

    prop_compose! {
        fn arb_topics_arg()(criteria in arb_topics_criteria_arg(arb_topics_matching_arg(), arb_topics_in_arg(), arb_topics_eq_arg())) -> String {
            format!("topics {}", criteria)
        }
    }

    prop_compose! {
        fn arb_key_name_arg()(key_name in "[a-zA-Z0-9_-]+") -> String { key_name.to_string() }
    }

    prop_compose! {
        fn arb_key_criteria_arg()(criteria in prop::sample::select(vec!["matching", "in", "eq"])) -> String { criteria.to_string() }
    }

    prop_compose! {
        fn arb_key_arg()(key_criteria in arb_key_criteria_arg(), key_name in arb_key_name_arg()) -> String { format!("key {} {}", key_criteria, key_name) }
    }

    prop_compose! {
        fn arb_regex()(payload_regex in "[a-zA-Z0-9_\\\\-\\\\.\\\\*\\\\+\\\\$]+") -> String { payload_regex.to_string() }
    }

    prop_compose! {
        fn arb_payload_criteria_arg()(criteria in prop::sample::select(vec!["containing"])) -> String { criteria.to_string() }
    }

    prop_compose! {
        fn arb_payload_arg()(criteria_name in arb_payload_criteria_arg(), payload_regex in arb_regex()) -> String {
            format!("for payload {} {}", criteria_name, payload_regex)
        }
    }

    prop_compose! {
        fn arb_query_arg()(cluster_arg in prop::option::of(arb_clusters_arg()), topics_arg in arb_topics_arg(), payload_arg in arb_payload_arg()) -> String {
            cluster_arg.map(|x| format!("query {} {} {}", x, topics_arg, payload_arg)).
            unwrap_or_else(|| format!("query {} {}", topics_arg, payload_arg))
        }
    }

    fn args_of(args: &str) -> Vec<String> {
        args.split(" ").map(|x| x.to_string()).collect()
    }
}
