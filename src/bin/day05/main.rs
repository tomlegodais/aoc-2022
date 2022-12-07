use itertools::{rev, Itertools};
use regex::Regex;
use std::collections::VecDeque;

pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day05/input.txt")?;
    let index = lines
        .iter()
        .position(|r: &String| r.contains("move"))
        .unwrap();

    let crate_rows = rev(&lines[..index - 2]).collect::<Vec<_>>();

    let total_stacks = crate_rows.len() + 1;
    let mut crate_stacks = Vec::new();
    for _ in 0..total_stacks {
        let empty_stack: VecDeque<String> = VecDeque::new();
        crate_stacks.push(empty_stack);
    }

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

    let is_digit = Regex::new(r"\d+")?;
    for movement_inst in &lines[index..] {
        let counts = is_digit
            .find_iter(movement_inst)
            .map(|mat| mat.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let num_crates = counts[0];
        let from_stack = counts[1] - 1; // adjust for zero based indexes
        let to_stack = counts[2] - 1; // adjust for zero based indexes

        for _ in 0..num_crates {
            let crate_id = &crate_stacks[from_stack]
                .pop_front()
                .expect("Unable to retrieve top of stack!");

            let _ = &crate_stacks[to_stack].push_front(crate_id.to_owned());
        }
    }

    for index in 0..crate_stacks.len() {
        let top_of_stack = &crate_stacks[index]
            .pop_front()
            .expect("Unable to retrieve top of stack!");

        print!("{}", top_of_stack)
    }

    Ok(())
}
