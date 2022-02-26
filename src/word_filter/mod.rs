use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

pub trait WordFilter {
    fn filter(&self, word_candidate: &str) -> bool;
}

lazy_static! {
    static ref WORD_REGEX: Regex = Regex::new("^[a-zA-Z]+$").unwrap();
}

pub struct RegexWordFilter;

impl WordFilter for RegexWordFilter {
    fn filter(&self, word_candidate: &str) -> bool {
        return WORD_REGEX.is_match(word_candidate);
    }
}

pub struct StopWordFilter<'a> {
    pub stopwords: HashSet<&'a str>,
}

impl WordFilter for StopWordFilter<'_> {
    fn filter(&self, word_candidate: &str) -> bool {
        return !self.stopwords.contains(&word_candidate);
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    // @formatter:off
    #[test_case("",         false;  "empty")]
    #[test_case(" ",        false;  "whitespace")]
    #[test_case("\t",       false;  "blank")]
    #[test_case("Hello1",   false;  "invalid")]
    #[test_case("hello",    true;   "lower case")]
    #[test_case("HELLO",    true;   "upper case")]
    #[test_case("hEllO",    true;   "mixed case")]
    // @formatter:on
    fn regex_word_filter(input: &str, is_expected_word: bool) {
        let word_filter = RegexWordFilter {};

        let is_word = word_filter.filter(input);

        assert_eq!(is_word, is_expected_word);
    }

    // @formatter:off
    #[test_case("word",     "stopword", true;   "not a stopword")]
    #[test_case("stopword", "stopword", false;  "stopword")]
    // @formatter:on
    fn stopwords_word_filter(input: &str, stopword: &str, is_expected_word: bool) {
        let word_filter = StopWordFilter {
            stopwords: vec!["asd"].into_iter().collect::<HashSet<&str>>()
        };

        let is_word = word_filter.filter(input);

        assert_eq!(is_word, is_expected_word);
    }
}
