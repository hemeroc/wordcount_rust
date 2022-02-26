#![feature(stdin_forwarders)]

use std::io;
use std::io::{stdin, Write};

use crate::word_counter::WordCounter;
use anyhow::Result;

mod word_counter;

fn main() -> Result<()> {
    print!("Enter text: ");
    io::stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let word_counter = WordCounter::new();
    let word_count = word_counter.count_words(&input)?.count;

    println!("Number of words: {word_count}");
    Ok(())
}
