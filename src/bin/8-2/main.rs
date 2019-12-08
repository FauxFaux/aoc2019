fn main() {
    let a: Vec<_> = include_str!("input").trim().chars().collect();

    let mut image = vec!['2'; 25 * 6];

    for layer in a.chunks(25 * 6).rev() {
        for (i, &c) in layer.iter().enumerate() {
            if c == '2' {
                continue;
            }
            image[i] = c;
        }
    }

    for line in image.chunks(25) {
        println!(
            "{}",
            line.iter()
                .map(|x| match x {
                    '0' => ' ',
                    '1' => 'X',
                    _ => unreachable!(),
                })
                .collect::<String>()
        );
    }
}
