use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

pub struct WordCount {
    pub count: usize,
}

lazy_static! {
    static ref WORD_REGEX: Regex = Regex::new("^[a-zA-Z]+$").unwrap();
}

pub struct WordCounter<'a> {
    pub word_regex: &'a Regex,
}

impl WordCounter<'_> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            word_regex: &WORD_REGEX,
        }
    }

    pub fn count_words(&self, input: &str) -> Result<WordCount> {
        let count = input
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|word_candidate| self.word_regex.is_match(word_candidate))
            .count();
        let word_count = WordCount { count };
        return Ok(word_count);
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    // @formatter:off
    #[test_case("",                 0;      "empty string")]
    #[test_case(" ",                0;      "blank string")]
    #[test_case("\t",               0;      "tab string")]
    #[test_case("Hello1",           0;      "invalid word")]
    #[test_case("Hello",            1;      "single valid word")]
    #[test_case("Hello1 World",     1;      "invalid and valid word")]
    #[test_case("Hello World",      2;      "multiple valid words")]
    // @formatter:on
    fn count_words(input: &str, expected_word_count: usize) -> Result<()> {
        let word_counter = WordCounter::new();

        let word_count = word_counter.count_words(input)?;

        assert_eq!(word_count.count, expected_word_count);

        Ok(())
    }
}
