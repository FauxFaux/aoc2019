use std::convert::TryFrom;

fn main() {
    let mut s = include_str!("input")
        .split(',')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i64>>();

    s.extend(vec![0; 10_000]);

    let a = interpret(vec![2], &mut s, &mut 0, &mut 0)[0];

    println!("{:?}", a);
}

fn param_ptr(s: &[i64], p: usize, n: usize, b: i64) -> usize {
    let op = s[p];
    assert!(op > 0);
    let mode = op / 10i64.pow((n + 1) as u32) % 10;
    match mode {
        0 => addr(s[p + n]),
        1 => p + n,
        2 => addr(s[p + n] + b),
        _ => unreachable!("bad opcode: {}", s[p]),
    }
}

fn param(s: &[i64], p: usize, n: usize, b: i64) -> i64 {
    s[param_ptr(s, p, n, b)]
}

fn addr(val: i64) -> usize {
    usize::try_from(val).expect("addr")
}

fn interpret(mut inputs: Vec<i64>, s: &mut [i64], p: &mut usize, b: &mut i64) -> Vec<i64> {
    let mut outputs = Vec::new();
    loop {
        let ins = s[*p];
        match ins % 100 {
            1 => {
                let dest = param_ptr(s, *p, 3, *b);
                s[dest] = param(s, *p, 1, *b) + param(s, *p, 2, *b);
                *p += 4;
            }

            2 => {
                let dest = param_ptr(s, *p, 3, *b);
                s[dest] = param(s, *p, 1, *b) * param(s, *p, 2, *b);
                *p += 4;
            }

            3 => {
                let dest = param_ptr(s, *p, 1, *b);
                s[dest] = inputs.remove(0);
                *p += 2;
            }

            4 => {
                outputs.push(param(s, *p, 1, *b));
                *p += 2;
            }

            5 => {
                if param(s, *p, 1, *b) != 0 {
                    *p = addr(param(s, *p, 2, *b));
                } else {
                    *p += 3;
                }
            }

            6 => {
                if param(s, *p, 1, *b) == 0 {
                    *p = addr(param(s, *p, 2, *b));
                } else {
                    *p += 3;
                }
            }

            7 => {
                let dest = param_ptr(s, *p, 3, *b);
                s[dest] = if param(s, *p, 1, *b) < param(s, *p, 2, *b) {
                    1
                } else {
                    0
                };
                *p += 4;
            }

            8 => {
                let dest = param_ptr(s, *p, 3, *b);
                s[dest] = if param(s, *p, 1, *b) == param(s, *p, 2, *b) {
                    1
                } else {
                    0
                };
                *p += 4;
            }

            9 => {
                *b += param(s, *p, 1, *b);
                *p += 2;
            }

            99 => return outputs,

            other => unreachable!("bad opcode: {}", other),
        }
    }
}
