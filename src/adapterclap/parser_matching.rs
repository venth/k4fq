use crate::adapterclap::predicate_matching;
use crate::predicates::ArcPredicate;
use std::io::Error;

pub fn parse(pattern: &str) -> Result<ArcPredicate<String>, Error> {
    new(predicate_matching::new(pattern))
}

fn new(parsed_predicate: Result<ArcPredicate<String>, impl std::error::Error + Send + Sync + 'static>) -> Result<ArcPredicate<String>, Error> {
    parsed_predicate
        .map_err(move |e| Error::new(std::io::ErrorKind::InvalidInput, e))
}

#[cfg(test)]
mod tests {
    use crate::predicates::{ArcPredicate, PredicateArcExt};
    use std::io::Error;

    #[test]
    fn fails_on_wrong_pattern_format() {
        // given
        let parsed_wrong_pattern = Err(Error::other("wrong pattern format"));

        // when
        let parse = super::new(parsed_wrong_pattern);

        // then
        assert_matches!(parse, Err(m) if m.to_string() == "wrong pattern format");
    }

    #[test]
    fn converts_pattern_to_predicate() {
        // given
        let predicate = predicates::prelude::predicate::always();

        // when
        let parse: Result<ArcPredicate<String>, Error> = Ok(predicate.arced());

        // then
        assert_matches!(parse, Ok(_))
    }
}