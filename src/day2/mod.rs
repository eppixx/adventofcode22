enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    /// what to choose based on enemy choice
    fn plan(&self, enemy: &Choice) -> Choice {
        match (enemy, self) {
            (enemy, Outcome::Draw) => *enemy,
            (Choice::Rock, Outcome::Lost) => Choice::Scissor,
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Paper, Outcome::Lost) => Choice::Rock,
            (Choice::Paper, Outcome::Win) => Choice::Scissor,
            (Choice::Scissor, Outcome::Lost) => Choice::Paper,
            (Choice::Scissor, Outcome::Win) => Choice::Rock,
        }
    }
}

impl From<char> for Outcome {
    fn from(input: char) -> Self {
        match input {
            'X' => Self::Lost,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            err => panic!("unexpected input: {}", err),
        }
    }
}

#[derive(Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Choice {
    /// outcome based on choice of both players
    fn play(&self, enemy: &Choice) -> Outcome {
        match (self, enemy) {
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Rock, Self::Paper) => Outcome::Win,
            (Self::Rock, Self::Scissor) => Outcome::Lost,
            (Self::Paper, Self::Rock) => Outcome::Lost,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Paper, Self::Scissor) => Outcome::Win,
            (Self::Scissor, Self::Rock) => Outcome::Win,
            (Self::Scissor, Self::Paper) => Outcome::Lost,
            (Self::Scissor, Self::Scissor) => Outcome::Draw,
        }
    }
}

impl From<char> for Choice {
    fn from(input: char) -> Self {
        match input {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissor,
            err => panic!("unexpected input: {}", err),
        }
    }
}

pub fn main() {
    let input = include_str!("./input");
    let games: Vec<(Choice, Choice)> = input
        .lines()
        .map(|game| {
            (
                Choice::from(game.chars().next().unwrap()),
                Choice::from(game.chars().nth(2).unwrap()),
            )
        })
        .collect();
    let score: i32 = games
        .iter()
        .map(|game| game.0.play(&game.1) as i32 + game.1 as i32)
        .sum();

    println!("\nday2");
    println!("score of gameplan {}", score);
    let games: Vec<(Choice, Outcome)> = input
        .lines()
        .map(|game| {
            (
                Choice::from(game.chars().next().unwrap()),
                Outcome::from(game.chars().nth(2).unwrap()),
            )
        })
        .collect();
    let score: i32 = games
        .iter()
        .map(|game| game.0.play(&game.1.plan(&game.0)) as i32 + game.1.plan(&game.0) as i32)
        .sum();
    println!("score of correct gameplan {}", score);
}
