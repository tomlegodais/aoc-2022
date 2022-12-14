pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day03/input.txt")?;
    let priorities = ('a'..='z')
        .chain('A'..='Z')
        .collect::<Vec<_>>();

    let total_sum = solve_part_01(&lines, &priorities);
    println!("Part One = {}", total_sum);

    let total_sum = solve_part_02(&lines, &priorities);
    println!("Part Two = {}", total_sum);

    Ok(())
}

fn solve_part_01(lines: &Vec<String>, priorities: &Vec<char>) -> usize {
    lines
        .iter()
        .fold(0, |acc, line| {
            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
            let priority = first_compartment
                .chars()
                .find_map(|x| second_compartment
                    .chars()
                    .find(|y| x == *y)
                    .map(|chr| priorities.iter().position(|x| *x == chr).unwrap() + 1))
                .unwrap();

            acc + priority
        })
}

fn solve_part_02(lines: &Vec<String>, priorities: &Vec<char>) -> usize {
    lines
        .chunks(3)
        .map(|chunk| aoc::find_common_char(&chunk.to_vec()))
        .filter(|common_char| common_char.is_some())
        .map(|common_char| common_char.and_then(|c| priorities.iter().position(|x| *x == c)).unwrap() + 1)
        .sum()
}