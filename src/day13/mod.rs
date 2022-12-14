#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum Signal {
    List(Vec<Signal>),
    Value(i32),
}

impl std::fmt::Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::List(a) => {
                write!(f, "{:?}", a)
            }
            Signal::Value(a) => write!(f, "{}", a),
        }
    }
}

impl Signal {
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (self, other) {
            (Signal::Value(a), Signal::Value(b)) => a.cmp(b),
            (Signal::Value(a), b) => Signal::List(vec![Signal::Value(*a)]).compare(b),
            (a, Signal::Value(b)) => a.compare(&Signal::List(vec![Signal::Value(*b)])),
            (Signal::List(a), Signal::List(b)) => {
                for i in 0..(a.len().max(b.len())) {
                    match (a.get(i), b.get(i)) {
                        (None, None) => unreachable!(),
                        (None, _) => return Ordering::Less,
                        (_, None) => return Ordering::Greater,
                        (Some(a), Some(b)) => match a.compare(b) {
                            Ordering::Equal => continue,
                            o => return o,
                        },
                    };
                }
                Ordering::Equal
            }
        }
    }

    fn is_right_order(&self, other: &Self) -> bool {
        matches!(self.compare(other), std::cmp::Ordering::Less)
    }
}

pub fn main() {
    println!("\nday 13");
    let input = include_str!("./input");
    let pairs = parse(input).unwrap().1;

    let count: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_i, (a, b))| a.is_right_order(b))
        .map(|(i, _)| i + 1)
        .sum();

    println!("sum of indeces with right order: {}", count);

    //part 2
    let decode_packets = vec![
        Signal::List(vec![Signal::Value(2)]),
        Signal::List(vec![Signal::Value(6)]),
    ];
    let mut lines: Vec<&Signal> = pairs
        .iter()
        .flat_map(|pair| vec![&pair.0, &pair.1])
        .collect();
    lines.push(&decode_packets[0]);
    lines.push(&decode_packets[1]);
    lines.sort_by(|a, b| a.compare(b));

    let distress_signal = (lines.iter().position(|a| **a == decode_packets[0]).unwrap() + 1)
        * (lines.iter().position(|a| **a == decode_packets[1]).unwrap() + 1);

    println!("decoder of key of distress signal {}", distress_signal);
}

fn parse(input: &str) -> nom::IResult<&str, Vec<(Signal, Signal)>> {
    use nom::branch::alt;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;
    use nom::Parser;
    use nom::{bytes::complete::tag, sequence::delimited};

    fn parse_line(input: &str) -> nom::IResult<&str, Signal> {
        // input is either a list or a value
        let (rest, signal) = alt((
            delimited(tag("["), separated_list0(tag(","), parse_line), tag("]")).map(Signal::List),
            (nom::character::complete::i32.map(Signal::Value)),
        ))(input)?;

        Ok((rest, signal))
    }

    // split by "/n/n" to get the pairs and then parse the lines with parse_line
    let (rest, signal) = separated_list0(
        tag("\n\n"),
        separated_pair(parse_line, tag("\n"), parse_line),
    )(input)?;

    Ok((rest, signal))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pair_rules() {
        let input = "[1,1,3,1]
[1,1,3,1,1]";

        let pairs = super::parse(input).unwrap().1;
        assert_eq!(pairs[0].0.is_right_order(&pairs[0].1), true);
    }
}
