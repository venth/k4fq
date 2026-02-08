use crate::adapterclap::cli_struct;
use crate::adapterclap::cli_struct::{
    MainCommands, QueryClustersCommands, QueryClustersTopicsCommands, QueryCommands,
    QueryTopicsCommands, QueryTopicsRecordsCommands, QueryTopicsRecordsForCommands,
    QueryTopicsRecordsKeyCommands, QueryTopicsRecordsPayloadCommands,
};
use crate::ports;
use crate::ports::Command;
use crate::predicates::{ArcPredicate, PredicateArcExt};
use clap::Parser;
use cli_struct::CliParser;
use std::path::PathBuf;

struct ClapCommandParser {}

pub fn new() -> impl ports::CommandParser {
    ClapCommandParser {}
}

impl ClapCommandParser {
    fn matched_records(
        &self,
        current_config: &PathBuf,
        clusters_predicate: &ArcPredicate<String>,
        topics_predicate: &ArcPredicate<String>,
        records: &QueryTopicsRecordsForCommands,
    ) -> Command {
        match records {
            QueryTopicsRecordsForCommands::ForClause { target } => match target {
                QueryTopicsRecordsCommands::Key { condition } => match condition {
                    QueryTopicsRecordsKeyCommands::Matching { predicate } => {
                        Command::Query {
                            config: current_config.clone(),
                            cluster_matcher: clusters_predicate.clone(),
                            topic_matcher: topics_predicate.clone(),
                            payload_matcher: predicate.clone(),
                        }
                    }
                    QueryTopicsRecordsKeyCommands::In { predicate } => {
                        Command::Query {
                            config: current_config.clone(),
                            cluster_matcher: clusters_predicate.clone(),
                            topic_matcher: topics_predicate.clone(),
                            payload_matcher: predicate.clone(),
                        }
                    }
                },
                QueryTopicsRecordsCommands::Payload { condition } => match condition {
                    QueryTopicsRecordsPayloadCommands::Containing { predicate } => {
                        Command::Query {
                            config: current_config.clone(),
                            cluster_matcher: clusters_predicate.clone(),
                            topic_matcher: topics_predicate.clone(),
                            payload_matcher: predicate.clone(),
                        }
                    }
                    QueryTopicsRecordsPayloadCommands::In { predicate } => {
                        Command::Query {
                            config: current_config.clone(),
                            cluster_matcher: clusters_predicate.clone(),
                            topic_matcher: topics_predicate.clone(),
                            payload_matcher: predicate.clone(),
                        }
                    }
                },
            },
        }
    }

    fn matched_topics(
        &self,
        current_config: &PathBuf,
        clusters_predicate: &ArcPredicate<String>,
        condition: &QueryTopicsCommands,
    ) -> Command {
        match condition {
            QueryTopicsCommands::Matching { predicate, records } => {
                self.matched_records(current_config, clusters_predicate, predicate, records)
            }
            QueryTopicsCommands::Eq { predicate, records } => {
                self.matched_records(current_config, clusters_predicate, predicate, records)
            }
            QueryTopicsCommands::In { predicate, records } => {
                self.matched_records(current_config, clusters_predicate, predicate, records)
            }
        }
    }
}

impl ports::CommandParser for ClapCommandParser {
    fn parse(&self, args: &Vec<String>) -> Command {
        CliParser::try_parse_from(args)
            .map(|a| match (a.parsed_command()) {
                MainCommands::Query { command } => match command {
                    QueryCommands::Clusters { condition } => match condition {
                        QueryClustersCommands::Matching { predicate, topics } => match topics {
                            QueryClustersTopicsCommands::Topics { condition } => {
                                self.matched_topics(a.current_config(), predicate, condition)
                            }
                        },
                        QueryClustersCommands::In { predicate, topics } => match topics {
                            QueryClustersTopicsCommands::Topics { condition } => {
                                self.matched_topics(a.current_config(), predicate, condition)
                            }
                        },

                        QueryClustersCommands::Eq { predicate, topics } => match topics {
                            QueryClustersTopicsCommands::Topics { condition } => {
                                self.matched_topics(a.current_config(), predicate, condition)
                            }
                        }
                    },
                    QueryCommands::Topics { condition } => self.matched_topics(
                        a.current_config(),
                        &predicates::prelude::predicate::always().arced(),
                        condition,
                    ),
                },
                MainCommands::ShowConfig => Command::ShowConfig {
                    config: a.current_config().clone(),
                },
            })
            .unwrap_or_else(|e| {
                eprint!("{}", e);
                return Command::skip_because_of(format!(
                    "Unrecognized command or flag. The details: {}",
                    e
                ));
            })
    }
}

#[cfg(test)]
mod tests {
    use super::ClapCommandParser;
    use crate::adapterclap::cli_struct::CliParser;
    use crate::ports::{Command, CommandParser};
    use log::{error, warn};
    use proptest::prelude::{prop, Strategy};
    use proptest::{prop_compose, prop_oneof, proptest};

    fn parser() -> impl CommandParser {
        ClapCommandParser {}
    }

    #[test]
    fn parses_help() {
        let a = args_of("query --help");
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
        let a = args_of("show-config");
        let result = parser().parse(&a);

        assert_matches!(result, Command::ShowConfig { config: c } if c.to_str().unwrap() == CliParser::default_config_path().to_str().unwrap());
    }

    #[test]
    fn prefers_config_flag_over_default_config_value_if_config_flag_is_passed() {
        let some_config_path = "/tmp/config.yaml";
        let a = args_of(
            format!(
                "--config {some_config_path} show-config",
                some_config_path = some_config_path
            )
            .as_str(),
        );
        let result = parser().parse(&a);

        assert_matches!(result, Command::ShowConfig { config: c } if c.to_str().unwrap() == some_config_path);
    }

    proptest! {
        #[test]
        fn returns_query_by_payload_command_if_query_by_payload_command_is_passed(query_arg in arb_query_arg()) {
            eprintln!("===> query_arg: {:?} <===", query_arg);
            let result = parser().parse(&args_of(query_arg.as_str()));

            assert_matches!(result, Command::Query { .. });
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

    fn arb_topics_criteria_arg(
        arb_matching: impl Strategy<Value = String>,
        arb_in: impl Strategy<Value = String>,
        arb_eq: impl Strategy<Value = String>,
    ) -> impl Strategy<Value = String> {
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
        fn arb_regex()(payload_regex in r"([a-zA-Z0-9_\.\$\-]|(\\[\.\*\+\$\-]))([a-zA-Z0-9_]|[\.\*\+\$\-]|(\\[\.\*\+\$\-]))*") -> String { payload_regex.to_string() }
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
