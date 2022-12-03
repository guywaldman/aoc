#[derive(Debug, Clone, Copy)]
pub(crate) enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid move: {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Outcome {
    Win,
    Loss,
    Draw,
}

pub(crate) fn get_outcome_for_round(p1: &Move, p2: &Move) -> Outcome {
    match (p1, p2) {
        (Move::Rock, Move::Scissors) => Outcome::Win,
        (Move::Rock, Move::Paper) => Outcome::Loss,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Paper, Move::Scissors) => Outcome::Loss,
        (Move::Scissors, Move::Paper) => Outcome::Win,
        (Move::Scissors, Move::Rock) => Outcome::Loss,
        _ => Outcome::Draw,
    }
}

pub(crate) fn get_score_for_round(outcome: &Outcome, mov: &Move) -> usize {
    let score_for_mov = match mov {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let score_for_outcome = match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    };
    score_for_mov + score_for_outcome
}

pub fn solve_part_01<'a>(input: &'a str) -> String {
    let moves = input
        .lines()
        .map(|line| {
            let moves_in_line = line.split_whitespace().map(Move::from).collect::<Vec<_>>();
            (moves_in_line[1], moves_in_line[0])
        })
        .collect::<Vec<_>>();

    let line_socres = moves
        .iter()
        .map(|(p1, p2)| get_score_for_round(&get_outcome_for_round(p1, p2), &p1))
        .collect::<Vec<_>>();

    let total_score = line_socres.iter().sum::<usize>();

    total_score.to_string()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {
        let input = "A Y
B X
C Z";
        assert_eq!(solve_part_01(input), "15");
    }
}
