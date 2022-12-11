use std::collections::HashSet;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        match input {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            input => unreachable!("unexpected input: {input}"),
        }
    }
}

pub fn main() {
    println!("\nday 9");
    let input = include_str!("./input");

    let instructions: Vec<(Direction, usize)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                Direction::from(split.next().unwrap()),
                str::parse::<usize>(split.next().unwrap()).unwrap(),
            )
        })
        .collect();

    // Coordinates are positive to the top right
    let mut state = ((0, 0), (0, 0));
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert(state.1);

    instructions.iter().for_each(|(direction, times)| {
        for _ in 0..*times {
            state = match direction {
                Direction::Right if state.0 .0 + 1 - state.1 .0 > 1 => {
                    // tail becomes old head
                    ((state.0 .0 + 1, state.0 .1), state.0)
                }
                Direction::Right => ((state.0 .0 + 1, state.0 .1), state.1),
                Direction::Left if state.0 .0 - 1 - state.1 .0 < -1 => {
                    //tail becomes old head
                    ((state.0 .0 - 1, state.0 .1), state.0)
                }
                Direction::Left => ((state.0 .0 - 1, state.0 .1), state.1),
                Direction::Up if state.0 .1 + 1 - state.1 .1 > 1 => {
                    //tail becomes old head
                    ((state.0 .0, state.0 .1 + 1), state.0)
                }
                Direction::Up => ((state.0 .0, state.0 .1 + 1), state.1),
                Direction::Down if state.0 .1 - 1 - state.1 .1 < -1 => {
                    //tail becomes old head
                    ((state.0 .0, state.0 .1 - 1), state.0)
                }
                Direction::Down => ((state.0 .0, state.0 .1 - 1), state.1),
            };
            tail_positions.insert(state.1);
        }
    });
    println!("number of unique tail positions: {}", tail_positions.len());

    //part 2
    let mut rope = vec![(0, 0); 10];
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    instructions.iter().for_each(|(direction, times)| {
        for _time in 0..*times {
            //move head
            rope[0] = match direction {
                Direction::Up => (rope[0].0, rope[0].1 + 1),
                Direction::Down => (rope[0].0, rope[0].1 - 1),
                Direction::Left => (rope[0].0 - 1, rope[0].1),
                Direction::Right => (rope[0].0 + 1, rope[0].1),
            };

            // move next knot accoring to pos of prev knot
            for i in 1..rope.len() {
                let prev = rope[i - 1];
                let current = rope[i];
                match (prev.0 - current.0, prev.1 - current.1) {
                    (x, y) if (-1..=1).contains(&x) && (-1..=1).contains(&y) => {}
                    (2, 0) => rope[i] = (current.0 + 1, current.1),
                    (2, -1) | (2, -2) | (1, -2) => rope[i] = (current.0 + 1, current.1 - 1),
                    (0, -2) => rope[i] = (current.0, current.1 - 1),
                    (-1, -2) | (-2, -2) | (-2, -1) => rope[i] = (current.0 - 1, current.1 - 1),
                    (-2, 0) => rope[i] = (current.0 - 1, current.1),
                    (-2, 1) | (-2, 2) | (-1, 2) => rope[i] = (current.0 - 1, current.1 + 1),
                    (0, 2) => rope[i] = (current.0, current.1 + 1),
                    (1, 2) | (2, 2) | (2, 1) => rope[i] = (current.0 + 1, current.1 + 1),
                    (x, y) => unreachable!("x diff: {x}, y diff: {y}"),
                }
            }
            // put tail in hashset
            tail_positions.insert(rope[9]);
        }
    });

    println!(
        "number of unique tail positions of new rope {}",
        tail_positions.len()
    );
}
