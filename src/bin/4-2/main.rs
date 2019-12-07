use itertools::Itertools;

fn main() {
    let a = (158126..=624574)
        .map(|p| format!("{}", p))
        .filter(|p| p.chars().tuple_windows().all(|(x, y)| x <= y))
        .filter(|p| {
            let p: Vec<_> = p.chars().collect();
            (p[0] == p[1] && p[1] != p[2])
                || (p[4] == p[5] && p[4] != p[3])
                || p.into_iter()
                    .tuple_windows()
                    .any(|(b, x, y, a)| x == y && x != b && y != a)
        })
        .count();

    println!("{}", a);
}
