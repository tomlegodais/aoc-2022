#[derive(PartialEq, Clone, Copy)]
enum Gesture {
    Rock,
    Paper,
    Scissor,
}

impl Gesture {
    fn parse_gesture(gesture_str: &str) -> Self {
        match gesture_str {
            "A" | "X" => Gesture::Rock,
            "B" | "Y" => Gesture::Paper,
            "C" | "Z" => Gesture::Scissor,
            _ => panic!("Unknown gesture (gesture_str={})!", gesture_str)
        }
    }

    fn parse_outcome(outcome: &Outcome, opponent_gesture: Gesture) -> Self {
        match outcome {
            Outcome::Win => opponent_gesture.get_weakness(),
            Outcome::Loss => opponent_gesture.get_superior(),
            Outcome::Draw => opponent_gesture,
        }
    }

    fn get_superior(&self) -> Self {
        match self {
            Gesture::Rock => Gesture::Scissor,
            Gesture::Paper => Gesture::Rock,
            Gesture::Scissor => Gesture::Paper
        }
    }

    fn get_weakness(&self) -> Self {
        match self {
            Gesture::Rock => Gesture::Paper,
            Gesture::Paper => Gesture::Scissor,
            Gesture::Scissor => Gesture::Rock,
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Gesture::Rock => 1,
            Gesture::Paper => 2,
            Gesture::Scissor => 3,
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn parse_outcome(outcome_str: &str) -> Self {
        match outcome_str {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unknown outcome (outcome_str={})!", outcome_str)
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day02/input.txt")?;

    let part_one_score = calculate_score(&lines, false);
    println!("Part One = {}", part_one_score);

    let part_two_score = calculate_score(&lines, true);
    println!("Part Two = {}", part_two_score);

    Ok(())
}

fn calculate_score(lines: &Vec<String>, part_two: bool) -> i32 {
    lines
        .iter()
        .map(|l: &String| {
            let mut split = l.split_whitespace();
            let opponent_gesture = Gesture::parse_gesture(split.next().unwrap());
            let self_gesture = if !part_two {
                Gesture::parse_gesture(split.next().unwrap())
            } else {
                let outcome = Outcome::parse_outcome(split.next().unwrap());
                Gesture::parse_outcome(&outcome, opponent_gesture)
            };

            (self_gesture, get_round_modifier(opponent_gesture, self_gesture))
        })
        .fold(0, |acc, (self_gesture, round_modifier)| acc + self_gesture.get_score() + round_modifier)
}

fn get_round_modifier(opponent_gesture: Gesture, self_gesture: Gesture) -> i32 {
    match (opponent_gesture, self_gesture) {
        (p1, p2) if p1 == p2 => 3,
        (p1, p2) if p1.get_weakness() == p2 => 6,
        (_, _) => 0
    }
}