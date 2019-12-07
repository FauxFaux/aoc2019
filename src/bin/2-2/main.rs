fn main() {
    let mut s = include_str!("input")
        .split(',')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();

    s[1] = 12;
    s[2] = 2;

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

    println!("{}", s[0]);
}
