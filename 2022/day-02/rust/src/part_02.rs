use crate::part_01::{get_outcome_for_round, get_score_for_round, Move, Outcome};

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome: {}", s),
        }
    }
}

fn infer_p1_move(p2_move: &Move, outcome: &Outcome) -> Move {
    match (p2_move, outcome) {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Rock, Outcome::Loss) => Move::Scissors,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Paper, Outcome::Loss) => Move::Rock,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Draw) => Move::Scissors,
        (Move::Scissors, Outcome::Loss) => Move::Paper,
    }
}

pub fn solve_part_02<'a>(input: &'a str) -> String {
    let moves = input
        .lines()
        .map(|line| {
            let moves_in_line: Vec<&str> = line.split_whitespace().collect::<Vec<_>>();
            let p2_move: Move = moves_in_line[0].into();
            let outcome: Outcome = moves_in_line[1].into();
            let p1_move = infer_p1_move(&p2_move, &outcome);
            (p1_move, p2_move)
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
        assert_eq!(solve_part_02(input), "12");
    }
}
