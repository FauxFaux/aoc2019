fn main() {
    let a: Vec<_> = include_str!("input").trim().chars().collect();

    let a = a
        .chunks(25 * 6)
        .min_by_key(|l| l.iter().filter(|&&v| v == '0').count())
        .expect("non-empty");

    let a = a.iter().filter(|&&c| c == '1').count() * a.iter().filter(|&&c| c == '2').count();

    println!("{:?}", a);
}
