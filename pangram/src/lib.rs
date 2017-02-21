use std::collections::HashSet;
use std::iter::FromIterator;
use std::ascii::AsciiExt;

pub fn is_pangram(sentence: &str) -> bool {
    sentence.to_lowercase().chars().filter(|c| c.is_alphabetic()).filter(|c| c.is_ascii())
    .collect::<HashSet<char>>() == HashSet::from_iter(ENGLISH_ALPHABET.chars())
}


const ENGLISH_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

