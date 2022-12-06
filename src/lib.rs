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

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    #[test]
    fn it_works() {
        assert_that!(true)
            .is_true();
    }
}
