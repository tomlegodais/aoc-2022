#[derive(PartialEq)]
enum Gesture {
    Rock,
    Paper,
    Scissor,
}

impl Gesture {
    fn parse_gesture(gesture_str: &str) -> Gesture {
        return match gesture_str {
            "A" | "X" => Gesture::Rock,
            "B" | "Y" => Gesture::Paper,
            "C" | "Z" => Gesture::Scissor,
            _ => panic!("Unknown gesture (gesture_str={})!", gesture_str)
        };
    }

    fn get_weakness(&self) -> Gesture {
        return match &self {
            Gesture::Rock => Gesture::Paper,
            Gesture::Paper => Gesture::Scissor,
            Gesture::Scissor => Gesture::Rock,
        };
    }

    fn get_score(&self) -> i32 {
        return match &self {
            Gesture::Rock => 1,
            Gesture::Paper => 2,
            Gesture::Scissor => 3,
        };
    }
}

pub fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines("src/bin/day02/input.txt")?;
    let total_score: i32 = lines
        .into_iter()
        .map(|l: String| {
            let mut split = l.split_whitespace();
            let opponent_gesture = Gesture::parse_gesture(split.next().unwrap());
            let self_gesture = Gesture::parse_gesture(split.next().unwrap());
            let score = self_gesture.get_score();
            let round_modifier = match (opponent_gesture, self_gesture) {
                (p1, p2) if p1 == p2 => 3,
                (p1, p2) if p1.get_weakness() == p2 => 6,
                (_, _) => 0
            };

            score + round_modifier
        })
        .fold(0, |acc, x| acc + x);

    println!("Part One = {}", total_score);

    Ok(())
}
