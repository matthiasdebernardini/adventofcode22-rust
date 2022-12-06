use std::str::FromStr;

use snafu::Snafu;

#[inpt::main]
fn main(i: String) {
    let mut score = 0;
    for l in i.lines() {
        let r = l.parse::<Round>().unwrap();
        score += score_round(&r.mine, &r.theirs)
    }
    print!("{score}");
}

struct Round {
    mine: Moves,
    theirs: Moves,
}

enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn score_round(mine: &Moves, theirs: &Moves) -> i32 {
    match (mine, theirs) {
        (Moves::Rock, Moves::Rock) => Moves::Rock as i32 + Outcome::Draw as i32,
        (Moves::Rock, Moves::Paper) => Moves::Rock as i32 + Outcome::Lose as i32,
        (Moves::Rock, Moves::Scissors) => Moves::Rock as i32 + Outcome::Win as i32,
        (Moves::Paper, Moves::Rock) => Moves::Paper as i32 + Outcome::Win as i32,
        (Moves::Paper, Moves::Paper) => Moves::Paper as i32 + Outcome::Draw as i32,
        (Moves::Paper, Moves::Scissors) => Moves::Paper as i32 + Outcome::Lose as i32,
        (Moves::Scissors, Moves::Rock) => Moves::Scissors as i32 + Outcome::Lose as i32,
        (Moves::Scissors, Moves::Paper) => Moves::Scissors as i32 + Outcome::Win as i32,
        (Moves::Scissors, Moves::Scissors) => Moves::Scissors as i32 + Outcome::Draw as i32,
    }
}

fn get_move(outcome: Outcome, moves: Moves)  -> Moves {
    match (outcome, moves) {
        (Outcome::Win, Moves::Rock) => Moves::Paper,
        (Outcome::Win, Moves::Paper) => Moves::Scissors,
        (Outcome::Win, Moves::Scissors) => Moves::Rock,
        (Outcome::Draw, Moves::Rock) => Moves::Rock,
        (Outcome::Draw, Moves::Paper) => Moves::Paper,
        (Outcome::Draw, Moves::Scissors) => Moves::Scissors,
        (Outcome::Lose, Moves::Rock) => Moves::Paper,
        (Outcome::Lose, Moves::Paper) => Moves::Scissors,
        (Outcome::Lose, Moves::Scissors) => Moves::Rock,
    }
}

#[derive(Debug, Snafu)]
struct ParseError {
    s: String,
}

impl FromStr for Outcome {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => ParseSnafu { s }.fail(),
        }
    }
}


impl FromStr for Moves {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Moves::Rock),
            "Y" => Ok(Moves::Paper),
            "Z" => Ok(Moves::Scissors),
            "A" => Ok(Moves::Rock),
            "B" => Ok(Moves::Paper),
            "C" => Ok(Moves::Scissors),
            _ => ParseSnafu { s }.fail(),
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_ascii_whitespace();
        let theirs = s.next().unwrap().parse::<Moves>().unwrap();
        let mine = s.next().unwrap().parse::<Moves>().unwrap();
        Ok(Round { mine, theirs })
    }
}
