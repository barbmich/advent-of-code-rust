use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Victory,
    Draw,
    Loss,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Game {
    opponent: Hand,
    me: Hand,
    outcome: Outcome,
}

impl Game {
    fn part_1(opp: &str, io: &str) -> Self {
        let opponent = get_hand(opp);
        let me = get_hand(io);
        let outcome = calc_output_pt_1(opponent, me);

        Self {
            opponent,
            me,
            outcome,
        }
    }

    fn part_2(opp: &str, out: &str) -> Self {
        let opponent = get_hand(opp);
        let outcome = get_outcome(out);
        let me = calc_output_pt_2(opponent, outcome);

        Self {
            opponent,
            me,
            outcome,
        }
    }
}

fn get_hand(string: &str) -> Hand {
    match string {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => unreachable!(),
    }
}

fn get_outcome(string: &str) -> Outcome {
    match string {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Victory,
        _ => unreachable!(),
    }
}

fn calc_output_pt_1(opponent: Hand, me: Hand) -> Outcome {
    match me {
        Hand::Paper => match opponent {
            Hand::Paper => Outcome::Draw,
            Hand::Scissors => Outcome::Loss,
            Hand::Rock => Outcome::Victory,
        },
        Hand::Rock => match opponent {
            Hand::Rock => Outcome::Draw,
            Hand::Paper => Outcome::Loss,
            Hand::Scissors => Outcome::Victory,
        },
        Hand::Scissors => match opponent {
            Hand::Scissors => Outcome::Draw,
            Hand::Rock => Outcome::Loss,
            Hand::Paper => Outcome::Victory,
        },
    }
}

fn calc_output_pt_2(opponent: Hand, outcome: Outcome) -> Hand {
    match outcome {
        Outcome::Loss => match opponent {
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
            Hand::Rock => Hand::Scissors,
        },
        Outcome::Draw => match opponent {
            Hand::Paper => Hand::Paper,
            Hand::Scissors => Hand::Scissors,
            Hand::Rock => Hand::Rock,
        },
        Outcome::Victory => match opponent {
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
            Hand::Rock => Hand::Paper,
        },
    }
}

fn calc_total(games_vector: Vec<Game>) -> i32 {
    games_vector
        .iter()
        .map(|game| {
            let hand_points = match game.me {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };
            let outcome_points = match game.outcome {
                Outcome::Loss => 0,
                Outcome::Draw => 3,
                Outcome::Victory => 6,
            };
            hand_points + outcome_points
        })
        .sum()
}

fn main() {
    let input = read_to_string("input/day-2.input").unwrap();
    let mut games_pt_1: Vec<Game> = vec![];

    for line in input.lines() {
        let vector: Vec<&str> = line.split(" ").collect();
        games_pt_1.push(Game::part_1(vector[0], vector[1]));
    }

    let part_1 = calc_total(games_pt_1);

    println!("part 1 - total points obtained: {}", part_1);

    let mut games_pt_2: Vec<Game> = vec![];
    for line in input.lines() {
        let vector: Vec<&str> = line.split(" ").collect();
        games_pt_2.push(Game::part_2(vector[0], vector[1]));
    }

    let part_2 = calc_total(games_pt_2);

    println!("part 2 - total points obtained: {}", part_2);
}
