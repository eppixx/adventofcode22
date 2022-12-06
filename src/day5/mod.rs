pub fn main() {
    let input = include_str!("input");

    // split input into two parts
    let mut split = input.split("\n\n");

    let mut stacks: Vec<Vec<char>> = vec![];
    split.next().unwrap().lines().for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                // expand len of stacks if new stack found
                if stacks.len() < i + 1 {
                    stacks.push(vec![]);
                }
                // should encounter either a char, a space or a number
                match c {
                    ' ' => {}
                    c if ('0'..'9').contains(&c) => {}
                    c => stacks[i].insert(0, c),
                }
            });
    });

    let instructions: Vec<(usize, usize, usize)> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            // line has format: ["move", "_", "from", "_", "to"]
            let parts = line.split(' ');
            let mut parts = parts.skip(1);
            let number = &str::parse::<usize>(parts.next().unwrap()).unwrap();
            let mut parts = parts.skip(1);
            let src = &str::parse::<usize>(parts.next().unwrap()).unwrap();
            let mut parts = parts.skip(1);
            let dest = &str::parse::<usize>(parts.next().unwrap()).unwrap();
            (*number, *src, *dest)
        })
        .collect();

    let mut stack_copy = stacks.clone();
    instructions.iter().for_each(|ins| {
        (1..=ins.0).step_by(1).for_each(|_| {
            let arm_content = stacks[ins.1 - 1].pop().unwrap();
            stacks[ins.2 - 1].push(arm_content);
        });
    });

    //part 2
    instructions.iter().for_each(|ins| {
        let stack_len = stack_copy[ins.1 - 1].len();
        let mut arm_content = stack_copy[ins.1 - 1].drain(stack_len - ins.0..).collect();
        stack_copy[ins.2 - 1].append(&mut arm_content);
    });

    let top_crates = |stacks: Vec<Vec<char>>| -> Vec<char> {
        stacks
            .iter()
            .map(|stack| stack.last().cloned().unwrap())
            .collect::<Vec<char>>()
    };

    println!("\nday 5");
    println!("the top of the stacks: {:?}", top_crates(stacks));
    println!(
        "the top of the stack with new crane is:  {:?}",
        top_crates(stack_copy)
    );
}
