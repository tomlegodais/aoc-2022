#![feature(step_trait)]

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Step;
use std::ops::Range;
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    Ok(lines)
}

pub fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;

    for (index, matched) in text.match_indices(r) {
        if last != index {
            result.push(&text[last..index]);
        }

        result.push(matched);
        last = index + matched.len();
    }

    if last < text.len() {
        result.push(&text[last..]);
    }

    result
}

pub fn rem_first_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_owned()
}

pub fn find_common_char(strings: &Vec<String>) -> Option<char> {
    ('a'..='z')
        .chain('A'..='Z')
        .find(|&c| strings.iter().all(|s| s.contains(c)))
}

pub fn check_range<T>(r1: &Range<T>, r2: &Range<T>) -> bool
    where T: Sized + PartialOrd
{
    (r1.start >= r2.start && r1.end <= r2.end) || (r1.start <= r2.start && r1.end >= r2.end)
}

pub fn check_any_inclusive<T>(r1: Range<T>, r2: Range<T>) -> bool
    where T: Sized + Step
{
    let mut r1_inclusive = r1.start..=r1.end;
    let mut r2_inclusive = r2.start..=r2.end;

    r1_inclusive.any(|x| r2_inclusive.contains(&x)) || r2_inclusive.any(|y| r1_inclusive.contains(&y))
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    #[test]
    fn it_works() {
        assert_that!(true).is_true();
    }
}
