use itertools::{rev, Itertools};
use regex::Regex;
use std::collections::VecDeque;

pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day05/input.txt")?;
    let index = lines
        .iter()
        .position(|r: &String| r.contains("move"))
        .unwrap();

    let solution_part_one = process_crates(&lines, &index, false)?;
    println!("Part One = {}", solution_part_one);

    let solution_part_two = process_crates(&lines, &index, true)?;
    println!("Part Two = {}", solution_part_two);

    Ok(())
}

fn process_crates(lines: &Vec<String>, index: &usize, part_two: bool) -> anyhow::Result<String> {
    let mut crate_stacks = create_stacks(&lines, index)?;
    let _ = parse_movements(&mut crate_stacks, &lines, index, part_two)?;

    Ok(read_top_stacks(&mut crate_stacks))
}

fn create_stacks(lines: &Vec<String>, index: &usize) -> anyhow::Result<Vec<VecDeque<String>>> {
    let crate_rows = rev(&lines[..index - 2]).collect::<Vec<_>>();
    let total_stacks = crate_rows.len() + 1;
    let mut crate_stacks = (0..total_stacks)
        .map(|_| VecDeque::new())
        .collect::<Vec<VecDeque<String>>>();

    let separator = Regex::new(r"\[[a-zA-Z]]")?;
    for row in crate_rows {
        let crates = aoc::split_keep(&separator, row)
            .into_iter()
            .map(|c: &str| c.to_string())
            .filter(|c: &String| c != " ")
            .collect_vec();

        let mut stack_index = 0;
        for cr in crates {
            let stack_data: (Option<String>, usize) = if cr.starts_with("[") {
                let chr = aoc::rem_first_last(cr);
                (Some(chr), 1)
            } else {
                (None, cr.len() / 4)
            };

            if let Some(chr) = stack_data.0 {
                crate_stacks[stack_index].push_front(chr);
            }

            stack_index += stack_data.1
        }
    }

    Ok(crate_stacks)
}

fn parse_movements(crate_stacks: &mut Vec<VecDeque<String>>, lines: &Vec<String>, index: &usize, part_two: bool) -> anyhow::Result<()> {
    let is_digit = Regex::new(r"\d+")?;
    for movement_inst in &lines[*index..] {
        let counts = is_digit
            .find_iter(movement_inst)
            .map(|mat| mat.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let num_crates = counts[0];
        let from_stack = counts[1] - 1; // adjust for zero based indexes
        let to_stack = counts[2] - 1; // adjust for zero based indexes

        move_crates(num_crates, from_stack, to_stack, crate_stacks, part_two);
    }

    Ok(())
}

fn move_crates(num_crates: usize, from_stack: usize, to_stack: usize, crate_stacks: &mut Vec<VecDeque<String>>, part_two: bool) {
    let vec = crate_stacks[from_stack].drain(..num_crates).collect_vec();
    let crates = match part_two {
        true => vec.into_iter().rev().collect_vec(),
        false => vec
    };

    for crate_id in crates {
        let _ = &crate_stacks[to_stack].push_front(crate_id);
    }
}

fn read_top_stacks(crate_stacks: &mut Vec<VecDeque<String>>) -> String {
    let mut output = String::new();
    for index in 0..crate_stacks.len() {
        let top_of_stack = &crate_stacks[index]
            .pop_front()
            .expect("Unable to retrieve top of stack!");

        output.push_str(top_of_stack.as_str());
    }

    output
}