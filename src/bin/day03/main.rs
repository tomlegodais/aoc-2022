pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day03/input.txt")?;
    let priorities = ('a'..='z')
        .chain('A'..='Z')
        .collect::<Vec<_>>();

    let total_sum = lines
        .iter()
        .fold(0, |acc, line| {
            let half_length = line.len() / 2;
            let first_compartment = &line[..half_length];
            let second_compartment = &line[half_length..];
            let priority = first_compartment
                .chars()
                .filter(|x| second_compartment.chars().any(|y| *x == y))
                .map(|chr| priorities.iter().position(|x| *x == chr).unwrap() + 1)
                .min()
                .unwrap();

            acc + priority
        });

    println!("Part One = {}", total_sum);

    Ok(())
}