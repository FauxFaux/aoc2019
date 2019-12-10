use std::convert::TryFrom;

fn main() {
    let mut m: Vec<Vec<bool>> = include_str!("input")
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().map(|c| '#' == c).collect::<Vec<_>>())
        .collect();

    let w = m[0].len();
    let h = m.len();

    let x = 30;
    let y = 34;

    let a = perms(i64(w + 1))
        .into_iter()
        .skip(1)
        .flat_map(|(dx, dy)| vec![(dx, dy), (-dx, dy), (dx, -dy), (-dx, -dy)])
        .filter(|&(dx, dy)| {
            let mut ax = x;
            let mut ay = y;

            let mut hit = false;

            loop {
                ax += dx;
                ay += dy;
                if ax < 0 || ay < 0 || ax >= i64(w) || ay >= i64(h) {
                    break;
                }

                let here = &mut m[usize(ay)][usize(ax)];
                hit |= *here;
                *here = false;
            }

            hit
        })
        .count();

    println!("{:?}", a);
}

fn i64(v: usize) -> i64 {
    i64::try_from(v).expect("fits in signed")
}

fn usize(v: i64) -> usize {
    usize::try_from(v).expect("positive")
}

fn perms(i: i64) -> Vec<(i64, i64)> {
    let mut v = Vec::new();
    for x in 0..i {
        for y in 0..i {
            v.push((x, y));
        }
    }
    v
}
