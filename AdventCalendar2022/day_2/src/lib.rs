use advent_utils::file_to_lines;

enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn from_char(c: char) -> Choice {
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("Unknown char for choice: {c}"),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss
}

impl Outcome {
    fn from_char(c: char) -> Outcome {
        match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Unknown char for outcome: {c}"),
        }
    }
}

use Choice::*;
use Outcome::*;

fn choice_score(key: &Choice) -> u64 {
    match key {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    }
}

fn outcome_score(opponent: &Choice, player: &Choice) -> u64 {
    match (opponent, player) {
        // Tie
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        // Win 
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        // Loss 
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
    }
}

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    let mut total_score : u64 = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        let mut chars = txt.chars()
            .filter(|c| *c != ' ');
        let opponent = &Choice::from_char(chars.next().unwrap());
        let player = &Choice::from_char(chars.next().unwrap());
        total_score += choice_score(player) + outcome_score(opponent, player);
    }

    println!("Total Score: {total_score}");

    Ok(())
}

fn player_choice_from_opponent_and_outcome(opponent: &Choice, outcome: &Outcome) -> Choice {
    match opponent {
        Rock => match outcome {
            Outcome::Win => Paper,
            Outcome::Draw => Rock,
            Outcome::Loss => Scissors,
        },
        Paper => match outcome {
            Outcome::Win => Scissors,
            Outcome::Draw => Paper,
            Outcome::Loss => Rock,
        },
        Scissors => match outcome {
            Outcome::Win => Rock,
            Outcome::Draw => Scissors,
            Outcome::Loss => Paper,
        },
    }
}

pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    let mut total_score : u64 = 0;
    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        let mut chars = txt.chars()
            .filter(|c| *c != ' ');
        let opponent = &Choice::from_char(chars.next().unwrap());
        let outcome = &Outcome::from_char(chars.next().unwrap());
        let player = &player_choice_from_opponent_and_outcome(opponent, outcome);
        total_score += choice_score(player) + outcome_score(opponent, player);
    }

    println!("Total Score: {total_score}");

    Ok(())
}
