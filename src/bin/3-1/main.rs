use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input");
    let (first, second) = input.split_at(input.find('\n').unwrap());
    let first = path(first);
    let second = path(second);

    let first = visits(&first);
    let second = visits(&second);

    let min = first
        .intersection(&second)
        .map(|(x, y)| x.abs() + y.abs())
        .min();

    println!("{:?}", min);
}

#[derive(Debug)]
struct Op {
    d: char,
    l: usize,
}

fn path(input: &str) -> Vec<Op> {
    input
        .split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| Op::new(v))
        .collect()
}

fn visits(input: &[Op]) -> HashSet<(i64, i64)> {
    let mut ret = HashSet::new();
    let (mut x, mut y) = (0, 0);
    for op in input {
        let (dx, dy) = match op.d {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            o => unreachable!("bad direction: {}", o),
        };
        for _ in 0..op.l {
            x += dx;
            y += dy;
            ret.insert((x, y));
        }
    }
    ret
}

impl Op {
    fn new(input: &str) -> Op {
        let (d, l) = split_first(input).expect("non-empty op");
        Op {
            d,
            l: usize::from_str(l).expect("number"),
        }
    }
}

fn split_first(input: &str) -> Option<(char, &str)> {
    let first = input.chars().next()?;
    Some((first, &input[first.len_utf8()..]))
}
