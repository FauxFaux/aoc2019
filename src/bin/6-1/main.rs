use std::collections::HashMap;

fn main() {
    let a: HashMap<&str, &str> = include_str!("input")
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_at(l.find(")").unwrap()))
        .map(|(a, b)| (&b[1..], a))
        .collect();

    let a = a
        .values()
        .map(|mut v| {
            let mut i = 0;

            loop {
                i += 1;
                v = match a.get(v) {
                    Some(v) => v,
                    None => break i,
                };
            }
        })
        .sum::<u64>();

    println!("{}", a);
}
