use predicates::prelude::predicate;
use predicates::{BoxPredicate, PredicateBoxExt};

pub fn new(pattern: &str, separator: &char) -> BoxPredicate<String> {
    if pattern.is_empty() {
        return predicate::never().boxed();
    }
    predicate::in_iter(
        pattern
            .split(*separator)
            .map(str::to_string)
            .filter(move |s| !s.is_empty()),
    )
    .boxed()
}

#[cfg(test)]
mod tests {
    use predicates::Predicate;
    use std::collections::HashMap;

    #[test]
    fn checks_if_input_matches_pattern() {
        // given
        let data = HashMap::from([
            (
                Input {
                    pattern: "".to_string(),
                    arg: "".to_string(),
                },
                false,
            ),
            (
                Input {
                    pattern: ",".to_string(),
                    arg: "".to_string(),
                },
                false,
            ),
            (
                Input {
                    pattern: "a,".to_string(),
                    arg: "".to_string(),
                },
                false,
            ),
            (
                Input {
                    pattern: "a,b,".to_string(),
                    arg: "".to_string(),
                },
                false,
            ),
            (
                Input {
                    pattern: "".to_string(),
                    arg: "topic".to_string(),
                },
                false,
            ),
            (
                Input {
                    pattern: "topic1".to_string(),
                    arg: "topic".to_string(),
                },
                false,
            ),
            (
                Input {
                    pattern: "topic1,topic2".to_string(),
                    arg: "topic".to_string(),
                },
                false,
            ),
            (
                Input {
                    pattern: "topic1,topic2".to_string(),
                    arg: "topic1".to_string(),
                },
                true,
            ),
            (
                Input {
                    pattern: "topic1,topic2".to_string(),
                    arg: "topic2".to_string(),
                },
                true,
            ),
        ]);

        data.iter().for_each(|(input, expected)| {
            // when
            let result = super::new(&input.pattern, &',').eval(&input.arg);

            // then
            assert_eq!(
                *expected, result,
                "Pattern: '{}' and arg: '{}' do match: {} but was: {}",
                input.pattern, input.arg, expected, result
            );
        });
    }

    #[derive(Debug, PartialEq, Hash, Eq)]
    struct Input {
        pattern: String,
        arg: String,
    }
}
