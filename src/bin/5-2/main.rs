use std::convert::TryFrom;

fn main() {
    let mut s = include_str!("input")
        .split(',')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i64>>();

    println!("{:?}", interpret(vec![5], &mut s));
}

fn param(s: &[i64], p: usize, n: usize) -> i64 {
    let op = s[p];
    assert!(op > 0);
    let mode = op / 10i64.pow((n + 1) as u32) % 10;
    match mode {
        0 => s[addr(s[p + n])],
        1 => s[p + n],
        _ => unreachable!("bad opcode: {}", s[p]),
    }
}

fn addr(val: i64) -> usize {
    usize::try_from(val).expect("addr")
}

fn interpret(mut inputs: Vec<i64>, s: &mut [i64]) -> Vec<i64> {
    let mut p = 0usize;
    let mut outputs = Vec::new();
    loop {
        let ins = s[p];
        match ins % 100 {
            1 => {
                let dest = addr(s[p + 3]);
                s[dest] = param(s, p, 1) + param(s, p, 2);
                p += 4;
            }

            2 => {
                let dest = addr(s[p + 3]);
                s[dest] = param(s, p, 1) * param(s, p, 2);
                p += 4;
            }

            3 => {
                let dest = addr(s[p + 1]);
                s[dest] = inputs.remove(0);
                p += 2;
            }

            4 => {
                outputs.push(param(s, p, 1));
                p += 2;
            }

            5 => {
                if param(s, p, 1) != 0 {
                    p = addr(param(s, p, 2));
                } else {
                    p += 3;
                }
            }

            6 => {
                if param(s, p, 1) == 0 {
                    p = addr(param(s, p, 2));
                } else {
                    p += 3;
                }
            }

            7 => {
                let dest = addr(param(s, p, 3));
                s[dest] = if param(s, p, 1) < param(s, p, 2) {
                    1
                } else {
                    0
                };
                p += 4;
            }

            8 => {
                let dest = addr(param(s, p, 3));
                s[dest] = if param(s, p, 1) == param(s, p, 2) {
                    1
                } else {
                    0
                };
                p += 4;
            }

            99 => break,

            other => unreachable!("bad opcode: {}", other),
        }
    }
    outputs
}
