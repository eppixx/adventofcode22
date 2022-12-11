#[derive(Clone, Debug)]
enum Operation {
    MulSelf,
    Add(u64),
    Mul(u64),
}

#[derive(Clone, Debug)]
struct Monkey {
    op: Operation,
    // divide by test.0; if true throw to test.1 else test.2
    test: (u64, usize, usize),
    inspected: u64,
}

fn get_input() -> (Vec<Monkey>, [Vec<u64>; 8]) {
    let monkeys: Vec<Monkey> = vec![
        Monkey {
            op: Operation::Mul(13),
            test: (19, 5, 6),
            inspected: 0,
        },
        Monkey {
            op: Operation::MulSelf,
            test: (7, 5, 0),
            inspected: 0,
        },
        Monkey {
            op: Operation::Add(6),
            test: (17, 1, 0),
            inspected: 0,
        },
        Monkey {
            op: Operation::Add(2),
            test: (13, 1, 2),
            inspected: 0,
        },
        Monkey {
            op: Operation::Add(3),
            test: (11, 3, 7),
            inspected: 0,
        },
        Monkey {
            op: Operation::Add(4),
            test: (2, 4, 6),
            inspected: 0,
        },
        Monkey {
            op: Operation::Add(8),
            test: (5, 4, 7),
            inspected: 0,
        },
        Monkey {
            op: Operation::Mul(7),
            test: (3, 2, 3),
            inspected: 0,
        },
    ];
    let holding: [Vec<u64>; 8] = [
        vec![72, 97],
        vec![55, 70, 90, 74, 95],
        vec![74, 97, 66, 57],
        vec![86, 54, 53],
        vec![50, 65, 78, 50, 62, 99],
        vec![90],
        vec![88, 92, 63, 94, 96, 82, 53, 53],
        vec![70, 60, 71, 69, 77, 70, 98],
    ];

    (monkeys, holding)
}

fn _get_test_input() -> (Vec<Monkey>, [Vec<u64>; 4]) {
    let monkeys: Vec<Monkey> = vec![
        Monkey {
            op: Operation::Mul(19),
            test: (23, 2, 3),
            inspected: 0,
        },
        Monkey {
            op: Operation::Add(6),
            test: (19, 2, 0),
            inspected: 0,
        },
        Monkey {
            op: Operation::MulSelf,
            test: (13, 1, 3),
            inspected: 0,
        },
        Monkey {
            op: Operation::Add(3),
            test: (17, 0, 1),
            inspected: 0,
        },
    ];
    let holding: [Vec<u64>; 4] = [
        vec![79, 98],
        vec![54, 65, 75, 74],
        vec![79, 60, 97],
        vec![74],
    ];

    (monkeys, holding)
}

pub fn main() {
    println!("\nday 11");
    // let input = include_str!("./input");

    let (mut monkeys, mut holding) = get_input();
    // let (mut monkeys, mut holding) = _get_test_input();

    for _round in 1..=20 {
        for (i, mut monkey) in monkeys.iter_mut().enumerate() {
            let current = holding[i].clone();
            for item in current {
                let worry = match monkey.op {
                    Operation::Add(num) => item + num,
                    Operation::Mul(num) => item * num,
                    Operation::MulSelf => item * item,
                };
                // round down
                let worry = (worry / 3) as u64;
                match worry % monkey.test.0 {
                    0 => holding[monkey.test.1].push(worry),
                    _ => holding[monkey.test.2].push(worry),
                }
                monkey.inspected += 1;
            }
            holding[i].clear();
        }

        // println!("After round {_round}");
        // for (i, monkey) in holding.iter().enumerate() {
        //     println!("Monkey {i}: {:?}", monkey);
        // }
    }

    // for (i, monkey) in monkeys.iter().enumerate() {
    //     println!("Monkey {} inspected items {} times.", i, monkey.inspected);
    // }

    let mut sorted = monkeys.clone();
    sorted.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    println!(
        "Level after 20 rounds {}",
        sorted.iter().take(2).map(|h| h.inspected).product::<u64>()
    );

    //part 2
    let (mut monkeys, mut holding) = get_input();
    // let (mut monkeys, mut holding) = _get_test_input();

    // chinese remainder theory
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // we can keep the worry level low by using modulo over the product of every
    // possible divisior of the monkeys (if they don't share a factor, which in our case
    // they don't)
    let crt_divisor: u64 = monkeys.iter().map(|m| m.test.0).product();
    for _round in 1..=10000 {
        for (i, mut monkey) in monkeys.iter_mut().enumerate() {
            // println!("money {i}");
            let current = holding[i].clone();
            for item in &current {
                let worry = match &monkey.op {
                    Operation::Add(num) => item + num,
                    Operation::Mul(num) => item * num,
                    Operation::MulSelf => item * item,
                };
                // apply chinese remainder theory
                let worry = worry % crt_divisor;
                // println!("worry {worry}, test with {}", monkey.test.0);
                match worry % monkey.test.0 {
                    0 => holding[monkey.test.1].push(worry),
                    _ => holding[monkey.test.2].push(worry),
                }
                monkey.inspected += 1;
            }
            holding[i].clear();
        }

        // match _round {
        //     1 | 19 | 20 | 1000 | 2000 | 3000 | 4000 | 5000 | 6000 | 7000 | 8000 | 9000 | 10000 => {
        //         println!("After round {_round}");
        //         for (i, monkey) in monkeys.iter().enumerate() {
        //             println!("Monkey {i} inspected items {} times.", monkey.inspected);
        //         }
        //     }
        //     _ => {}
        // }
    }

    let mut sorted = monkeys.clone();
    sorted.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    println!(
        "Level after 10000 rounds and new rules {:?}",
        sorted.iter().take(2).map(|h| h.inspected).product::<u64>()
    );
}
