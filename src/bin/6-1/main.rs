use std::collections::HashMap;

fn main() {
    let a: HashMap<&str, &str> = include_str!("input")
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().split_at(3))
        .map(|(a, b)| (a, &b[1..]))
        .collect();

    let a = a
        .values()
        .map(|mut v| {
            let mut i = 0;

            loop {
                v = match a.get(v) {
                    Some(v) => v,
                    None => break i,
                };
                i += 1;
            }
        })
        .sum::<u64>();

    println!("{}", a);
}
