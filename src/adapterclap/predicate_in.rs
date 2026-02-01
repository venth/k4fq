use itertools::Itertools;
use crate::adapterclap::predicate_error::PredicateError;
use crate::predicates::{ArcPredicate, PredicateArcExt};
use predicates::prelude::predicate;

pub fn new(
    pattern: &str,
    separator: &char,
) -> Result<ArcPredicate<String>, impl std::error::Error> {
    let elements = pattern
        .split(*separator)
        .map(str::to_string)
        .filter(move |s| !s.is_empty())
        .collect::<Vec<String>>();
    if elements.is_empty() {
        return Err(PredicateError::from(
            format!("Pattern {} cannot be empty. Expected comma separated list of values", pattern).to_string(),
        ));
    }
    Ok(predicate::in_iter(elements.iter().cloned()).arced())
}

#[cfg(test)]
mod tests {
    use predicates::Predicate;

    #[test]
    fn fails_on_wrong_pattern_because_expect_comma_separated_value() {
        // given
        let wrong_patterns = vec!["", ",", ",,"];

        // expect
        wrong_patterns.iter().for_each(|pattern| {
            assert_matches!(
                super::new(pattern, &','),
                Err(_),
                "Pattern: '{}' should be invalid",
                pattern
            )
        });
    }

    #[test]
    fn creates_predicate_from_valid_pattern() {
        // given
        let valid_pattern = vec!["a,b,c", "a", "a,b"];

        // expect
        valid_pattern.iter().for_each(|pattern| {
            assert!(
                super::new(pattern, &',').is_ok(),
                "Pattern: '{}' should be valid",
                pattern
            )
        });
    }

    #[test]
    fn matches_elements_in_pattern() {
        // given
        let valid_el1 = "el1";
        let valid_el2 = "el2";
        let valid_el3 = "el3";
        let pattern = super::new(
            format!("{},{},{}", valid_el1, valid_el2, valid_el3).as_str(),
            &',',
        )
        .expect("Pattern should be valid");

        // when
        assert!(pattern.eval(&valid_el1.to_string()));
        assert!(pattern.eval(&valid_el2.to_string()));
        assert!(pattern.eval(&valid_el3.to_string()));
    }

    #[test]
    fn matches_element_in_pattern() {
        // given
        let valid_el1 = "el1";

        // expect
        let _valid_patterns = vec!["{}", "{},"].iter()
            .map(|p| p.replace("{}", valid_el1))
            .map(|p| super::new(p.as_str(), &',').expect("Pattern should be valid"))
            .for_each(|pattern| assert!(pattern.eval(&valid_el1.to_string()), "Pattern: '{}' should match element: '{}'", pattern, valid_el1));
    }

    #[test]
    fn does_not_match_element_outside_pattern() {
        // given
        let valid_el = "valid";
        let invalid_el = "invalid";

        // expect
        let _valid_patterns = vec!["{}", "{},"].iter()
            .map(|p| p.replace("{}", valid_el))
            .map(|p| super::new(p.as_str(), &',').expect("Pattern should be valid"))
            .for_each(|pattern| {
                assert_eq!(pattern.eval(&invalid_el.to_string()), false, "Pattern: '{}' shouldn't match element: '{}'", pattern, invalid_el)
            });
    }
}
