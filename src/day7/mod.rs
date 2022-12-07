#[derive(Clone, Debug)]
enum Node {
    File(String, usize),
    Folder(String),
}

pub fn main() {
    let input = include_str!("./input");

    use std::collections::HashMap;
    let mut fs: HashMap<Vec<String>, Vec<Node>> = HashMap::new();
    let mut pwd: Vec<String> = vec![];
    let mut lines = input.lines().peekable();
    loop {
        let line = match lines.next() {
            None => break,
            Some(line) => line,
        };
        let split_line: Vec<&str> = line.split(' ').collect();
        match &split_line[..] {
            ["$", "cd", ".."] => {
                let _ = pwd.pop();
            }
            ["$", "cd", folder] => pwd.push(String::from(*folder)),
            ["$", "ls"] => loop {
                match lines.peek() {
                    None => break,
                    Some(peeks) => {
                        let split_line: Vec<&str> = peeks.split(' ').collect();
                        match split_line[..] {
                            ["dir", folder] => {
                                lines.next();
                                let folder = Node::Folder(String::from(folder));
                                match fs.contains_key(&pwd) {
                                    false => {
                                        fs.insert(pwd.clone(), vec![folder]);
                                    }
                                    true => {
                                        let mut v = fs.get(&pwd).unwrap().clone();
                                        v.push(folder);
                                        fs.insert(pwd.clone(), v.to_vec());
                                    }
                                }
                            }
                            [number, name] => {
                                lines.next();
                                let file = Node::File(
                                    String::from(name),
                                    str::parse::<usize>(number).unwrap(),
                                );
                                match fs.contains_key(&pwd) {
                                    false => {
                                        fs.insert(pwd.clone(), vec![file]);
                                    }
                                    true => {
                                        let mut v = fs.get(&pwd).unwrap().clone();
                                        v.push(file);
                                        fs.insert(pwd.clone(), v.to_vec());
                                    }
                                }
                            }
                            _ => break,
                        }
                    }
                }
            },
            input => unreachable!("unexpected input: {:?}", input),
        }
    }

    fn calc_size(pwd: &Vec<String>, fs: &HashMap<Vec<String>, Vec<Node>>) -> usize {
        let mut sum = 0;
        match fs.get(pwd) {
            None => {}
            Some(nodes) => {
                for node in nodes {
                    match node {
                        Node::File(_, size) => sum += size,
                        Node::Folder(name) => {
                            let mut pwd2 = pwd.clone();
                            pwd2.push(String::from(name));
                            sum += calc_size(&pwd2, fs);
                        }
                    }
                }
            }
        }
        sum
    }

    let sum: usize = fs
        .iter()
        .map(|(pwd, _)| {
            let size = calc_size(pwd, &fs);
            if size <= 100000 {
                size
            } else {
                0
            }
        })
        .sum();

    //part 2
    let space_limit = 70000000;
    let space_left = space_limit - calc_size(&vec![String::from("/")], &fs);
    let space_needed = 30000000 - space_left;

    let deleted_dir = fs
        .iter()
        .fold(None, |acc, (pwd, _)| match (acc, calc_size(pwd, &fs)) {
            (None, size) if size > space_needed => Some(pwd),
            (Some(acc), size) if size > space_needed && size < calc_size(acc, &fs) => Some(pwd),
            _ => acc,
        });

    println!("\nday 7");
    println!("sum of size of folders with content over 10000: {}", sum);
    println!(
        "min sized folder marked for deletion for updating: {}",
        calc_size(deleted_dir.unwrap(), &fs)
    );
}
