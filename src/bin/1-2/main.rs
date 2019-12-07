fn main() {
    println!(
        "{}",
        include_str!("input")
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.trim().parse().unwrap())
            .map(fuel_for)
            .sum::<u64>()
    )
}

fn fuel_for(mass: u64) -> u64 {
    if 0 == mass {
        return 0;
    }

    let immediate = (mass / 3).saturating_sub(2);
    immediate + fuel_for(immediate)
}
