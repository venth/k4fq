use crate::adapterclap::predicate_error::PredicateError;
use crate::predicates::{ArcPredicate, PredicateArcExt};
use predicates::prelude::predicate;

pub fn new(pattern: String) -> Result<ArcPredicate<String>, impl std::error::Error> {
    if (pattern.is_empty()) {
        return Err(PredicateError::from("Pattern cannot be empty".to_string()));
    }
    Ok(predicate::eq(pattern).arced())
}

#[cfg(test)]
mod tests {
    use predicates::Predicate;

    #[test]
    fn fails_on_wrong_pattern() {
        // given
        let wrong_patterns = vec![""];

        // expect
        wrong_patterns.iter().for_each(|pattern| {
            assert_matches!(
                super::new(pattern.to_string()),
                Err(_),
                "Pattern: '{}' should be invalid",
                pattern
            )
        });
    }

    #[test]
    fn creates_predicate_from_valid_pattern() {
        // given
        let valid_pattern = vec!["abc", " ", "  "];

        // expect
        valid_pattern.iter().for_each(|pattern| {
            assert!(
                super::new(pattern.to_string()).is_ok(),
                "Pattern: '{}' should be valid",
                pattern
            )
        });
    }

    #[test]
    fn matches_given_pattern() {
        // given
        let valid_pattern = vec!["abc", " ", "  "];

        // expect
        valid_pattern.iter().for_each(|pattern| {
            assert_eq!(true,
                       super::new(pattern.to_string())
                           .map(|a| a.eval(&pattern.to_string()))
                           .unwrap())
        });
    }
}
