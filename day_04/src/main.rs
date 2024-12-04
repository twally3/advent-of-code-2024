fn main() {
    let lines = std::fs::read_to_string("./input.txt")
        .expect("File should exist")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let char_grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    let cols = lines[0].len();
    let rows = lines.len();

    let convs = [
        [['M', '\0', 'M'], ['\0', 'A', '\0'], ['S', '\0', 'S']],
        [['M', '\0', 'S'], ['\0', 'A', '\0'], ['M', '\0', 'S']],
        [['S', '\0', 'M'], ['\0', 'A', '\0'], ['S', '\0', 'M']],
        [['S', '\0', 'S'], ['\0', 'A', '\0'], ['M', '\0', 'M']],
    ];

    let mut total = 0;

    for i in 0..rows {
        for j in 0..cols {
            let i_hat = i + 2;
            let j_hat = j + 2;

            if (i_hat >= rows || j_hat >= cols) {
                continue;
            }

            for conv in convs {
                let mut matches = true;
                for (i, sub_rows) in char_grid[i..=i_hat].iter().enumerate() {
                    let c = conv[i];
                    let sub_col = &sub_rows[j..=j_hat]
                        .iter()
                        .enumerate()
                        .map(|(i2, ch)| match i {
                            1 => match i2 {
                                0 | 2 => '\0',
                                1 => *ch,
                                _ => unreachable!(),
                            },
                            0 | 2 => match i2 {
                                0 | 2 => *ch,
                                1 => '\0',
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>();

                    if matches {
                        matches = c == **sub_col;
                    }
                }
                if matches {
                    total += 1;
                }
            }
        }
    }
    dbg!(total);
}

fn main2() {
    let lines = std::fs::read_to_string("./input.txt")
        .expect("File should exist")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut total = 0;

    // HORIZONTAL
    for line in lines.iter() {
        total += line.chars().collect::<Vec<_>>().windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );

        total += line.chars().rev().collect::<Vec<_>>().windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );
    }

    // VERTICAL
    let cols = lines[0].len();
    let rows = lines.len();

    let mut result = vec![String::with_capacity(rows); cols];
    let char_grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    for i in 0..cols {
        for j in (0..rows).rev() {
            result[i].push(char_grid[j][i]);
        }
    }

    for line in result.iter() {
        total += line.chars().collect::<Vec<_>>().windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );

        total += line.chars().rev().collect::<Vec<_>>().windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );
    }

    // DIAGONAL TOP LEFT
    let mut a = Vec::new();

    for k in 0..=(rows + cols - 2) {
        let mut b = Vec::new();
        for j in 0..=k {
            let i = k - j;

            if i < rows && j < cols {
                //println!("({i}, {k})");
                b.push(char_grid[i][j]);
            }
        }
        a.push(b);
    }

    for line in a {
        total += line.windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );

        total += line.into_iter().rev().collect::<Vec<_>>().windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );
    }

    // DIAGONAL BOTTOM LEFT
    let char_grid_2: Vec<Vec<char>> = result.iter().map(|s| s.chars().collect()).collect();

    let mut a = Vec::new();

    for k in 0..=(rows + cols - 2) {
        let mut b = Vec::new();
        for j in 0..=k {
            let i = k - j;

            if i < rows && j < cols {
                b.push(char_grid_2[i][j]);
            }
        }
        a.push(b);
    }

    for line in a {
        total += line.windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );

        total += line.into_iter().rev().collect::<Vec<_>>().windows(4).fold(
            0,
            |acc, x| if String::from_iter(x) == "XMAS" { 1 } else { 0 } + acc,
        );
    }

    dbg!(total);
}
