use regex::Regex;

pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day04/input.txt")?;
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;

    let full_overlap_total = check_overlap(&lines, &regex, true);
    println!("Part One = {}", full_overlap_total);

    let overlap_total = check_overlap(&lines, &regex, false);
    println!("Part Two = {}", overlap_total);

    Ok(())
}

fn check_overlap(lines: &Vec<String>, regex: &Regex, full_overlap: bool) -> u32 {
    lines
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
            match full_overlap {
                true if aoc::check_range(&first_range, &second_range) => Some(1),
                false if aoc::check_any_inclusive(first_range, second_range) => Some(1),
                _ => None
            }
        })
        .sum::<u32>()
}