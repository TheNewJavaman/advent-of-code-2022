fn main() {
    let rounds = include_str!("input/day2.txt").split('\n').map(|line| {
        let mut cols = line.chars();
        let col1 = cols.next().unwrap();
        let col2 = cols.nth(1).unwrap();
        (col1, col2)
    });

    let part1: u32 = rounds
        .clone()
        .map(|(opp, you)| {
            let (opp, you) = (Strat::from(opp), Strat::from(you));
            let outcome = if you == opp {
                Outcome::Draw
            } else {
                match (you, opp) {
                    (Strat::Rock, Strat::Paper)
                    | (Strat::Paper, Strat::Scissors)
                    | (Strat::Scissors, Strat::Rock) => Outcome::Loss,
                    (Strat::Rock, Strat::Scissors)
                    | (Strat::Paper, Strat::Rock)
                    | (Strat::Scissors, Strat::Paper) => Outcome::Win,
                    _ => unreachable!(),
                }
            };
            you as u32 + outcome as u32
        })
        .sum();
    println!("1. {part1}");

    let part2: u32 = rounds
        .map(|(col1, col2)| {
            let (opp, outcome) = (Strat::from(col1), Outcome::from(col2));
            let you = if outcome == Outcome::Draw {
                opp
            } else {
                match (outcome, opp) {
                    (Outcome::Loss, Strat::Paper) | (Outcome::Win, Strat::Scissors) => Strat::Rock,
                    (Outcome::Loss, Strat::Scissors) | (Outcome::Win, Strat::Rock) => Strat::Paper,
                    (Outcome::Loss, Strat::Rock) | (Outcome::Win, Strat::Paper) => Strat::Scissors,
                    _ => unreachable!(),
                }
            };
            you as u32 + outcome as u32
        })
        .sum();
    println!("2. {part2}");
}

#[derive(PartialEq, Clone, Copy)]
enum Strat {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for Strat {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Strat::Rock,
            'B' | 'Y' => Strat::Paper,
            'C' | 'Z' => Strat::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }
}
