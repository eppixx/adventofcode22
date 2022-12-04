pub fn main() {
    let input = include_str!("./input");

    //e.g. [[2, 4], [4, 5]] is one assignment
    let assignments: Vec<Vec<Vec<usize>>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .map(|value| value.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect();
    let overlaps: usize = assignments
        .iter()
        .map(|group| {
            let elf1 = &group[0];
            let elf2 = &group[1];
            //check for each elf assigment if it contains the other elfs assigment
            usize::from(
                elf1[0] <= elf2[0] && elf1[1] >= elf2[1]
                    || elf2[0] <= elf1[0] && elf2[1] >= elf1[1],
            )
        })
        .sum();

    //part 2
    let overall: usize = assignments
        .iter()
        .map(|group| {
            let elf1 = &group[0];
            let elf2 = &group[1];
            //check for each value in assigment of elf1 if its in elf2's assigment
            usize::from(
                (elf1[0]..=elf1[1])
                    .into_iter()
                    .any(|i| i >= elf2[0] && i <= elf2[1]),
            )
        })
        .sum();

    println!("\nday 4");
    println!("overlapping assignments: {}", overlaps);
    println!("overlapping assignments overall {}", overall);
}
