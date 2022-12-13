use regex::Regex;

pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day04/input.txt")?;
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
    let total_overlap = lines
        .iter()
        .filter_map(|line| {
            let range_data = regex.captures(&line)
                .map(|x| (1..3)
                    .chain(3..5)
                    .map(|n| x[n].parse::<u32>().unwrap())
                    .collect::<Vec<_>>())
                .unwrap();

            let first_range = range_data[0]..range_data[1];
            let second_range = range_data[2]..range_data[3];
            match (aoc::check_range(&first_range, &second_range), aoc::check_range(&second_range, &first_range)) {
                (true, _) | (_, true) => Some(1),
                _ => None
            }
        })
        .sum::<u32>();

    println!("Part One = {}", total_overlap);

    Ok(())
}