use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    #[test]
    fn it_works() {
        assert_that!(true).is_true();
    }
}
