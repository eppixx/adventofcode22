use std::collections::HashSet;

pub fn main() {
    let input = include_str!("./input");

    let mut visible: HashSet<(char, usize, usize)> = HashSet::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, candidate)| {
            // collect ranges from candidate to the edges
            let left = &input.lines().nth(i).unwrap()[0..j];
            let right = &input.lines().nth(i).unwrap()[j + 1..];
            let above: Vec<Option<char>> = input
                .lines()
                .take(i)
                .map(|line| line.chars().nth(j))
                .collect();
            let below: Vec<Option<char>> = input
                .lines()
                .skip(i + 1)
                .map(|line| line.chars().nth(j))
                .collect();

            // edges
            if left.is_empty()
                || right.is_empty()
                || above.get(0).is_none()
                || below.get(0).is_none()
            // inside
                || left.chars().all(|c| c < candidate)
                || right.chars().all(|c| c < candidate)
                || above.iter().all(|&c| c.unwrap() < candidate)
                || below.iter().all(|&c| c.unwrap() < candidate)
            {
                visible.insert((candidate, j, i));
            }
        })
    });

    // part 2
    let mut scores: Vec<(char, usize, usize, u32)> = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, candidate)| {
            // collect ranges from candidate to the edges
            let left = &input.lines().nth(y).unwrap()[0..x];
            let right = &input.lines().nth(y).unwrap()[x + 1..];
            let above: Vec<Option<char>> = input
                .lines()
                .take(y)
                .map(|line| line.chars().nth(x))
                .collect();
            let below: Vec<Option<char>> = input
                .lines()
                .skip(y + 1)
                .map(|line| line.chars().nth(x))
                .collect();

            // rules
            // if tree at edge => 0
            // if tree smaller than self => + 1
            // if tree bigger than self => stop and + 1
            let left = left
                .chars()
                .rev()
                .fold((false, 0), |acc, c| match acc.0 {
                    true => (true, acc.1),
                    false if c < candidate => (false, acc.1 + 1),
                    false => (true, acc.1 + 1),
                })
                .1;
            let right = right
                .chars()
                .fold((false, 0), |acc, c| match acc.0 {
                    true => (true, acc.1),
                    false if c < candidate => (false, acc.1 + 1),
                    false => (true, acc.1 + 1),
                })
                .1;
            let above = above
                .iter()
                .rev()
                .fold((false, 0), |acc, c| match (acc.0, c) {
                    (false, None) => (true, acc.1),
                    (false, Some(c)) if c < &candidate => (false, acc.1 + 1),
                    (false, Some(_)) => (true, acc.1 + 1),
                    (true, _) => (true, acc.1),
                })
                .1;
            let below = below
                .iter()
                .fold((false, 0), |acc, c| match (acc.0, c) {
                    (false, None) => (true, acc.1),
                    (false, Some(c)) if c < &candidate => (false, acc.1 + 1),
                    (false, Some(_)) => (true, acc.1 + 1),
                    (true, _) => (true, acc.1),
                })
                .1;
            scores.push((candidate, x, y, left * right * above * below));
        })
    });

    println!("\nday 8");
    println!("number of visible trees from outside is: {}", visible.len());
    println!(
        "the tree with the best scenic score is: {}",
        scores.iter().map(|(_, _, _, score)| score).max().unwrap()
    );
}
