#![feature(stdin_forwarders)]
#![feature(box_syntax)]

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, stdin, Write};

use anyhow::Result;

use crate::word_counter::WordCounter;
use crate::word_filter::{RegexWordFilter, StopWordFilter, WordFilter};

mod word_counter;
mod word_filter;

fn main() -> Result<()> {
    print!("Enter text: ");
    io::stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let file = File::open("stopwords.txt")?;
    let stopwords = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().as_str())
        .collect::<HashSet<&str>>();

    let word_filter: Vec<Box<dyn WordFilter>> = vec![
        box RegexWordFilter {},
        box StopWordFilter { stopwords },
    ];

    let word_counter = WordCounter::new(word_filter);
    let word_count = word_counter.count_words(&input)?.count;

    println!("Number of words: {word_count}");
    Ok(())
}
