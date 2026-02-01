use crate::adapterclap::predicate_error;
use crate::predicates::{ArcPredicate, PredicateArcExt};
use predicate_error::PredicateError;
use predicates::prelude::predicate;
use predicates::Predicate;
use std::error;

pub fn new(pattern: &str) -> Result<ArcPredicate<String>, impl error::Error> {
    Ok(pattern)
        .and_then(move |p| if p.is_empty() {
            Err("Pattern cannot be empty".to_string())
        } else {
            Ok(p)
        })
        .and_then(move |p| predicate::str::is_match(p).map_err(move |e| e.to_string()))
        .map(move |p| predicate::function(move |s: &String| {
            p.eval(s.as_str())
        }))
        .map(|p| p.arced())
        .map_err(move |msg| PredicateError::from(msg))
}


#[cfg(test)]
mod tests {
    use predicates::Predicate;

    #[test]
    fn fails_because_of_wrong_regex_pattern() {
        // given
        let wrong_regex = vec!["", "(.", ".)", "[."];

        wrong_regex.iter().for_each(|pattern| {
            // when
            let result = super::new(&pattern);

            // then
            assert_matches!(result, Err(_), "Pattern '{}' should not be a valid regex, but it seems that it is.", pattern);
        });
    }

    #[test]
    fn matches_regex_pattern() {
        // given
        let pattern = ".?opi.?";

        // and
        let predicate = super::new(&pattern).expect("Pattern should be a valid regex.");

        // when
        let result = predicate.eval(&"topic".to_string());

        // then
        assert!(result);
    }

    #[test]
    fn not_matches_arg_with_regex_pattern() {
        // given
        let pattern = "^.?opi.?$";

        // and
        let predicate = super::new(&pattern).expect("Pattern should be a valid regex.");

        // when
        let result = predicate.eval(&"topic1".to_string());

        // then
        assert_eq!(result, false);
    }

    #[test]
    fn matches_arg_with_regex_pattern_partially() {
        // given
        let pattern = ".?opi.?";

        // and
        let predicate = super::new(&pattern).expect("Pattern should be a valid regex.");

        // when
        let result = predicate.eval(&"u__topic_1".to_string());

        // then
        assert!(result);
    }
}