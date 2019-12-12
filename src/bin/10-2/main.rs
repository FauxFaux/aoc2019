use std::convert::TryFrom;

use itertools::Itertools;
use noisy_float::prelude::*;

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
    let mut d: Vec<_> = perms(i64(w + 1))
        .into_iter()
        .skip(1)
        .flat_map(|(dx, dy)| vec![(dx, dy), (-dx, dy), (dx, -dy), (-dx, -dy)])
        .sorted()
        .unique()
        .map(|(dx, dy)| (r32((dy as f32).atan2(dx as f32)), (dx, dy)))
        .sorted_by_key(|&(a, (dx, dy))| (a, -dx * dx - dy * dy))
        .collect();

    // BORROW CHECKER

    // I'm worried this is my best programming joke of the year.
    let pi = d.len();
    d.rotate_right(pi / 4);

    d.reverse();

    println!("{:?}", d);

    let mut last = R32::default();

    let a = d
        .into_iter()
        .cycle()
        .filter(|&(a, (dx, dy))| {
            if last == a {
                return false;
            }
            last = a;
            let mut ax = x;
            let mut ay = y;

            loop {
                ax += dx;
                ay += dy;
                if ax < 0 || ay < 0 || ax >= i64(w) || ay >= i64(h) {
                    return false;
                }

                let here = &mut m[usize(ay)][usize(ax)];
                if *here {
                    *here = false;
                    return true;
                }
                *here = false;
            }

            false
        })
        .enumerate()
        .inspect(|&(n, (a, (dx, dy)))| println!("{}: {},{} {:?}", n, x + dx, y + dy, (dx, dy)))
        .nth(200)
        .map(|(_, (_, (dx, dy)))| (x + dx) * 100 + (y + dy));

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
