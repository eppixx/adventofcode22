enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    /// what to choose based on enemy choice
    fn plan(&self, enemy: &Choice) -> Choice {
        match (enemy, self) {
            (enemy, Outcome::Draw) => enemy.clone(),
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

#[derive(Clone, PartialEq, Eq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Choice {
    /// outcome based on choice of both players
    fn against(&self, enemy: &Choice) -> Outcome {
        match (self, enemy) {
            (choice1, choice2) if choice1 == choice2 => Outcome::Draw,
            (Self::Rock, Self::Paper) => Outcome::Lost,
            (Self::Rock, Self::Scissor) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissor) => Outcome::Lost,
            (Self::Scissor, Self::Rock) => Outcome::Lost,
            (Self::Scissor, Self::Paper) => Outcome::Win,
            _ => unreachable!("logic error"),
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

    let score: i32 = input
        .lines()
        .map(|game| {
            let enemy = Choice::from(game.chars().next().unwrap());
            let my_choice = Choice::from(game.chars().nth(2).unwrap());
            let outcome = my_choice.against(&enemy);
            // win/draw/loose + my_choice
            outcome as i32 + my_choice as i32
        })
        .sum();
    let score_correct_plan: i32 = input
        .lines()
        .map(|game| {
            let enemy = Choice::from(game.chars().next().unwrap());
            let planned_outcome = Outcome::from(game.chars().nth(2).unwrap());
            let planned_choice = planned_outcome.plan(&enemy);
            // win/draw/loose + my_choice
            planned_choice.against(&enemy) as i32 + planned_choice as i32
        })
        .sum();

    println!("\nday2");
    println!("score of gameplan {}", score);
    println!("score of correct gameplan {}", score_correct_plan);
}
