use std::fs;

fn main() {
    println!(
        "{}",
        include_str!("input")
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.trim().parse().unwrap())
            .map(|d: u64| d / 3 - 2)
            .sum::<u64>()
    )
}
