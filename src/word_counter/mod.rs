use anyhow::Result;

use crate::word_filter::WordFilter;

pub struct WordCount {
    pub count: usize,
}

pub struct WordCounter {
    pub word_filter: Vec<Box<dyn WordFilter>>,
}

impl WordCounter {
    #[inline]
    #[must_use]
    pub fn new(word_filter: Vec<Box<dyn WordFilter>>) -> Self {
        Self { word_filter }
    }

    pub fn count_words(&self, input: &str) -> Result<WordCount> {
        let count = input
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|word_candidate| self.word_filter.iter().all(
                |word_filter| word_filter.filter(word_candidate)))
            .count();
        let word_count = WordCount { count };
        return Ok(word_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysTrueWordFilter;

    impl WordFilter for AlwaysTrueWordFilter {
        fn filter(&self, _word_candidate: &str) -> bool {
            return true;
        }
    }

    struct AlwaysFalseWordFilter;

    impl WordFilter for AlwaysFalseWordFilter {
        fn filter(&self, _word_candidate: &str) -> bool {
            return false;
        }
    }

    #[test]
    fn word_is_counted_if_all_filters_return_true() -> Result<()> {
        let word_filter: Vec<Box<dyn WordFilter>> = vec![
            box AlwaysTrueWordFilter {},
            box AlwaysTrueWordFilter {},
        ];
        let word_counter = WordCounter::new(word_filter);

        let word_count = word_counter.count_words("word1 word2")?.count;

        assert_eq!(word_count, 2);

        Ok(())
    }

    #[test]
    fn word_is_not_counted_if_any_filter_returns_false() -> Result<()> {
        let word_filter: Vec<Box<dyn WordFilter>> = vec![
            box AlwaysTrueWordFilter {},
            box AlwaysFalseWordFilter {},
        ];
        let word_counter = WordCounter::new(word_filter);

        let word_count = word_counter.count_words("word1 word2")?.count;

        assert_eq!(word_count, 0);

        Ok(())
    }
}
