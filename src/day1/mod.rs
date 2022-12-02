pub fn main() {
    let input = include_str!("./input");

    let elf_backpacks: Vec<&str> = input.split("\n\n").collect();
    let mut elf_calories: Vec<i32> = elf_backpacks
        .iter()
        .map(|backpack| backpack.lines().flat_map(str::parse::<i32>).sum())
        .collect();
    // sort from most to least calories
    elf_calories.sort_by(|a, b| b.cmp(a));

    println!("\nday 1");
    println!("most calories are: {}", elf_calories.first().unwrap());
    println!(
        "top 3 calories are: {}",
        elf_calories.iter().take(3).sum::<i32>()
    );
}
