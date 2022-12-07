use itertools::sorted;

pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day01/input.txt")?;
    let grouped_calories = lines
        .split(|line| line.is_empty())
        .filter(|sp| !sp.is_empty())
        .map(|sp| {
            sp.iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let sorted_calories: _ = sorted(
        grouped_calories
            .into_iter()
            .map(|cg| cg.iter().sum::<i32>()),
    );

    let maximum_calories = sorted_calories.clone().last().unwrap();
    let nth_calories: _ = sorted_calories.clone().rev().take(3).sum::<i32>();

    println!("part one = {}", maximum_calories);
    println!("part two = {}", nth_calories);

    Ok(())
}
