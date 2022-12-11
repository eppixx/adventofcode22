#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

pub fn main() {
    println!("\nday 10");
    // let input = include_str!("./input-test");
    let input = include_str!("./input");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|lines| {
            let mut split = lines.split_whitespace();
            match &split.next() {
                Some("noop") => Instruction::Noop,
                Some("addx") => {
                    Instruction::Addx(str::parse::<i32>(split.next().unwrap()).unwrap())
                }
                input => unreachable!("unexpected input {:?}", input),
            }
        })
        .collect();

    let mut register = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;
    instructions.iter().for_each(|instruction| {
        match instruction {
            Instruction::Noop => cycle += 1,
            Instruction::Addx(number) => {
                cycle += 1;
                let pipeline = register + number;
                match cycle {
                    19 | 59 | 99 | 139 | 179 | 219 => {
                        signal_strength += (cycle + 1) * register;
                    }
                    _ => {}
                }

                cycle += 1;
                register = pipeline;
            }
        }

        match cycle {
            19 | 59 | 99 | 139 | 179 | 219 => {
                signal_strength += (cycle + 1) * register;
            }
            _ => {}
        }
    });

    println!(
        "The sum of the six signal strengths is: {}",
        signal_strength
    );

    // part 2
    let mut crt = ['.'; 40 * 6];
    let mut register = 1;
    let mut cycle = 0;
    instructions.iter().for_each(|instruction| {
        cycle += 1;
        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(number) => {
                let pipeline = register + number;
                match register - (cycle % 40) {
                    i if i >= -1 && i <= 1 => {
                        crt[(cycle - 1) as usize] = '#';
                    }
                    _ => {}
                };

                cycle += 1;
                register = pipeline;
            }
        }
        match register - (cycle % 40) {
            i if i >= -1 && i <= 1 => {
                crt[(cycle - 1) as usize] = '#';
            }
            _ => {}
        };
    });

    // TODO there is a one by one error somewhere
    // the output is wrong but in a readable state
    for (i, value) in crt.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        }
        print!("{value}");
    }
}
