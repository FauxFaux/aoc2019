fn main() {
    let s = include_str!("input")
        .split(',')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut s = s.clone();
            s[1] = noun;
            s[2] = verb;
            interpret(&mut s);
            if 19690720 == s[0] {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}

fn interpret(s: &mut [usize]) {
    let mut p = 0usize;
    loop {
        match s[p] {
            1 => {
                let dest = s[p + 3];
                s[dest] = s[s[p + 1]] + s[s[p + 2]];
                p += 4;
            }

            2 => {
                let dest = s[p + 3];
                s[dest] = s[s[p + 1]] * s[s[p + 2]];
                p += 4;
            }

            99 => break,

            other => unreachable!("bad opcode: {}", other),
        }
    }
}
