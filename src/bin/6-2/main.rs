use std::collections::HashMap;

fn main() {
    let a: HashMap<&str, &str> = include_str!("input")
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_at(l.find(")").unwrap()))
        .map(|(a, b)| (&b[1..], a))
        .collect();

    let you = path(&a, "YOU");
    let san = path(&a, "SAN");

    for (i, place) in you.iter().enumerate() {
        match san.iter().position(|v| v == place) {
            Some(s) => {
                println!("{}", i + s);
                break;
            }
            None => continue,
        }
    }
}

// the worst lifetime restrictions
fn path<'a>(m: &HashMap<&'a str, &'a str>, mut src: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    while let Some(&v) = m.get(src) {
        ret.push(v);
        src = v;
    }
    ret
}
