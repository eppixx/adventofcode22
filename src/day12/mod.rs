pub fn main() {
    println!("\nday 12");
    let input = include_str!("./input");

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let find_pos = |find: &char| -> (i32, i32) {
        let line: (usize, &Vec<char>) = map
            .iter()
            .enumerate()
            .find(|(_y, line)| line.contains(find))
            .unwrap();
        (
            line.1
                .iter()
                .enumerate()
                .find(|(_x, c)| c == &find)
                .unwrap()
                .0 as i32,
            line.0 as i32,
        )
    };
    let start = find_pos(&'S');
    let end = find_pos(&'E');

    //generate edges
    let mut edges: Vec<((i32, i32), (i32, i32))> = vec![];
    for y in 0..map.len() as i32 {
        for x in 0..map[0].len() as i32 {
            // generate potential edges
            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let new_point = (x - dir.0, y - dir.1);
                // check if new_point is outside of map
                if new_point.0 < 0
                    || new_point.1 < 0
                    || new_point.0 == map[0].len() as i32
                    || new_point.1 == map.len() as i32
                {
                    continue;
                }
                //check for elevation rules
                let point_c = match map[y as usize][x as usize] {
                    'S' => 'a',
                    'E' => 'z',
                    c => c,
                };
                let new_point_c = match map[new_point.1 as usize][new_point.0 as usize] {
                    'S' => 'a',
                    'E' => 'z',
                    c => c,
                };
                if point_c as i32 + 1 >= new_point_c as i32 {
                    // new_point is a edge for (x, y)
                    edges.push(((x, y), new_point));
                }
            }
        }
    }

    let graph = petgraph::graphmap::DiGraphMap::<_, ()>::from_edges(&edges);
    // this output can be put into a graphviz editor and be visualized
    // println!(
    //     "graphviz {:?}",
    //     petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
    // );
    let result = petgraph::algo::dijkstra(&graph, start, Some(end), |_| 1);

    println!("The shortest valid path is {}", result[&end]);

    // part 2
    // insert the edges the in the inverse way
    let edges: Vec<((i32, i32), (i32, i32))> = edges.iter().map(|(a, b)| (*b, *a)).collect();
    let graph = petgraph::graphmap::DiGraphMap::<_, ()>::from_edges(edges);
    let result = petgraph::algo::dijkstra(&graph, end, None, |_| 1);

    // gather all possible end points with elevation 'a'
    let mut all_a = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'a' {
                all_a.push((x as i32, y as i32));
            }
        }
    }
    let min = all_a
        .iter()
        // filter the 'a' which are not reachable because None is shorter than Some(_)
        .filter(|p| result.get(p).is_some())
        .map(|p| result.get(p))
        .min()
        .unwrap();

    println!(
        "The shortes path from the lowest elevation 'a' is {:?}",
        min
    );
}
