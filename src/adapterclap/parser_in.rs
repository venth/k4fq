use crate::adapterclap::predicate_in;
use crate::predicates::{ArcPredicate, PredicateArcExt};
use std::io::Error;

pub fn parse(arg: &str) -> Result<ArcPredicate<String>, Error> {
    new(predicate_in::new(arg, &',').arced())(arg)
}

fn new(predicate: impl predicates::Predicate<String> + Send + Sync + Clone + 'static) -> impl Fn(&str) -> Result<ArcPredicate<String>, Error> {
    move |arg| {
        if arg.trim().is_empty() {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "'in' expects a comma separated list of values"));
        }
        Ok(predicate.clone().arced())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fails_on_empty_arg() {
        // given
        let predicate = predicates::prelude::predicate::always();
        // and
        let parse_fn = super::new(predicate);

        vec!["", " "].iter().for_each(|arg| {
            // when
            let result = parse_fn(arg);

            // then
            assert!(result.is_err(), "expected error for arg: '{}'", arg);
        })
    }

    #[test]
    fn converts_arg_to_predicate() {
        // given
        let predicate = predicates::prelude::predicate::always();
        // and
        let parse_fn = super::new(predicate);

        vec!["a", "a,b"].iter().for_each(|arg| {
            // when
            let result = parse_fn(arg);

            // then
            assert!(result.is_ok());
            assert_matches!(result.unwrap(), predicate)
        })
    }
}
