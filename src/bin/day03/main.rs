pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day03/input.txt")?;
    let priorities = ('a'..='z')
        .chain('A'..='Z')
        .collect::<Vec<_>>();

    let mut total_sum = 0;
    for line in lines {
        let half_length = line.len() / 2;
        let first_compartment = &line[..half_length];
        let second_compartment = &line[half_length..];
        let priority = first_compartment
            .chars()
            .filter(|x| second_compartment.chars().any(|y| *x == y))
            .map(|chr| priorities.iter().position(|x| *x == chr).unwrap() + 1)
            .min()
            .unwrap();

        total_sum += priority;
    }

    println!("Part One = {}", total_sum);

    Ok(())
}