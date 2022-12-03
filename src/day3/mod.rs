pub fn main() {
    let input = include_str!("./input");

    let compartments: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect();

    let convert_prio = |c: char| -> usize {
        match c {
            c if ('a'..='z').contains(&c) => c as usize - 'a' as usize + 1,
            c if ('A'..='Z').contains(&c) => c as usize - 'A' as usize + 27,
            input => unreachable!("unecpected input: {}", input),
        }
    };

    let priority: usize = compartments
        .iter()
        .map(|(first, sec)| {
            // make every characterr in first unique
            let mut first: Vec<char> = first.chars().collect();
            first.sort();
            first.dedup();

            first
                .iter()
                .map(|c| {
                    if sec.find(|v| c == &v).is_some() {
                        convert_prio(*c)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();

    //part 2
    let mut groups = vec![];
    let mut counter = 0;
    let mut group = vec![];
    for line in input.lines() {
        counter += 1;
        group.push(line);
        if counter >= 3 {
            counter = 0;
            groups.push(group);
            group = vec![];
        }
    }

    let badges: usize = groups
        .iter()
        .map(|group| {
            // make every characterr in first unique
            let mut first: Vec<char> = group[0].chars().collect();
            first.sort();
            first.dedup();

            first
                .iter()
                .map(|c| {
                    if group[1].find(|v| c == &v).is_some() && group[2].find(|v| c == &v).is_some()
                    {
                        convert_prio(*c)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();

    println!("\nday 3");
    println!("the sum of priority is: {}", priority);
    println!("the sum of priority of badges: {}", badges)
}
