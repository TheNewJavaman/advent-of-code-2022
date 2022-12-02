fn main() {
    let input = include_str!("input/day2.txt");
    let rounds = input.split('\n').map(|line| {
        let mut cols = line.chars();
        let col1 = cols.next().unwrap();
        let col2 = cols.nth(1).unwrap();
        (col1, col2)
    });

    let mixup = rounds
        .clone()
        .map(|(col1, col2)| {
            let (opp, you) = (Strat::from(col1), Strat::from(col2));
            let outcome = if you == opp {
                Outcome::Draw
            } else {
                match (you, opp) {
                    (Strat::Rock, Strat::Paper) => Outcome::Loss,
                    (Strat::Rock, Strat::Scissors) => Outcome::Win,
                    (Strat::Paper, Strat::Rock) => Outcome::Win,
                    (Strat::Paper, Strat::Scissors) => Outcome::Loss,
                    (Strat::Scissors, Strat::Rock) => Outcome::Loss,
                    (Strat::Scissors, Strat::Paper) => Outcome::Win,
                    _ => unreachable!(),
                }
            };
            you as u32 + outcome as u32
        })
        .sum::<u32>();
    println!("1. {mixup}");

    let actual = rounds
        .map(|(col1, col2)| {
            let (opp, outcome) = (Strat::from(col1), Outcome::from(col2));
            let you = if outcome == Outcome::Draw {
                opp
            } else {
                match (outcome, opp) {
                    (Outcome::Loss, Strat::Rock) => Strat::Scissors,
                    (Outcome::Loss, Strat::Paper) => Strat::Rock,
                    (Outcome::Loss, Strat::Scissors) => Strat::Paper,
                    (Outcome::Win, Strat::Rock) => Strat::Paper,
                    (Outcome::Win, Strat::Paper) => Strat::Scissors,
                    (Outcome::Win, Strat::Scissors) => Strat::Rock,
                    _ => unreachable!(),
                }
            };
            you as u32 + outcome as u32
        })
        .sum::<u32>();
    println!("2. {actual}");
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
