use std::{fs, task::Context};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Moves {
    fn Elve_Move(input: char) -> Self {
        match input {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("shouldn't happens"),
        }
    }
    fn My_Move(input: char) -> Self {
        match input {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("shouldn't happens"),
        }
    }

    fn Calcluate_Score(Elve: Self, Me: Self) -> u8 {
        let mut score = 0;

        //add score for move taken
        score += Me as u8;

        //calculate score for winning
        score += match (Elve, Me) {
            (Moves::Rock, Moves::Rock) => 3,
            (Moves::Rock, Moves::Paper) => 6,
            (Moves::Rock, Moves::Scissors) => 0,
            (Moves::Paper, Moves::Rock) => 0,
            (Moves::Paper, Moves::Paper) => 3,
            (Moves::Paper, Moves::Scissors) => 6,
            (Moves::Scissors, Moves::Rock) => 6,
            (Moves::Scissors, Moves::Paper) => 0,
            (Moves::Scissors, Moves::Scissors) => 3,
        };

        score
    }

    fn Get_Move_Needed(Elve_Move: Self, Score_Wanted: char) -> Self {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum Outcome {
            Lose,
            Draw,
            Win,
        }
        let outcome = match Score_Wanted {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("shouldn't Hapen"),
        };

        match outcome {
            Outcome::Lose => match Elve_Move {
                Moves::Rock => Moves::Scissors,
                Moves::Paper => Moves::Rock,
                Moves::Scissors => Moves::Paper,
            },
            Outcome::Draw => Elve_Move,
            Outcome::Win => match Elve_Move {
                Moves::Rock => Moves::Paper,
                Moves::Paper => Moves::Scissors,
                Moves::Scissors => Moves::Rock,
            },
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut score = 0;

    for line in contents.lines() {
        let Elve_move = Moves::Elve_Move(line.chars().nth(0).unwrap());

        let My_move = Moves::Get_Move_Needed(Elve_move, line.chars().nth(2).unwrap());

        score += Moves::Calcluate_Score(Elve_move, My_move) as i64;
    }

    println!("{score}");
}
