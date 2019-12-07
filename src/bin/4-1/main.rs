use itertools::Itertools;

fn main() {
    let a = (158126..=624574)
        .map(|p| format!("{}", p))
        .filter(|p| p.chars().tuple_windows().all(|(x, y)| x <= y))
        .filter(|p| p.chars().tuple_windows().any(|(x, y)| x == y))
        .count();

    println!("{}", a);
}
