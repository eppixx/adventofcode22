pub fn main() {
    let input = include_str!("./input");

    let marker = input
        .chars()
        .enumerate()
        .fold(
            (0, ' ', ' ', ' '),
            |(marker, third, second, first), (i, c)| {
                if marker == 0
                    && [third, second, first].iter().all(|c| c != &' ')
                    && third != second
                    && third != first
                    && third != c
                    && second != first
                    && second != c
                    && first != c
                {
                    (i, second, first, c)
                } else {
                    (marker, second, first, c)
                }
            },
        )
        .0;

    //part 2
    let new_marker = input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, slice)| {
            let mut slice = slice.to_vec();
            slice.sort();
            let len = slice.len();
            slice.dedup();
            len == slice.len()
        })
        .unwrap()
        .0;

    println!("\nday 6");
    println!("start of packet marker at {}", marker + 1);
    println!("start of new packet marker at {}", new_marker + 14);
}
