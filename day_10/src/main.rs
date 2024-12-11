fn calculate_routes(map: &Vec<Vec<u32>>, trailhead: (usize, usize)) -> usize {
    //let mut nines = std::collections::HashSet::new();
    let mut queue = vec![trailhead];
    let mut total = 0;

    while let Some((x, y)) = queue.pop() {
        let node = map[x][y];

        for i in x.saturating_sub(1)..=std::cmp::min(map.len() - 1, x + 1) {
            for j in y.saturating_sub(1)..=std::cmp::min(map[0].len() - 1, y + 1) {
                if !(x == i || y == j) {
                    continue;
                }

                let n = map[i][j];

                if n > node && n.abs_diff(node) == 1 {
                    //if n == 9 && !nines.contains(&(i, j)) {
                    if n == 9 {
                        //nines.insert((i, j));
                        total += 1;
                    } else {
                        queue.push((i, j));
                    }
                }
            }
        }
    }

    //nines.len()
    total
}

fn main() {
    let lines = std::fs::read_to_string("./input.txt")
        .expect("File should exist")
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut trailheads = Vec::new();
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    let mut total = 0;
    for trailhead in trailheads {
        let x = calculate_routes(&lines, trailhead);
        total += x;
        dbg!(x);
    }

    dbg!(total);
}
