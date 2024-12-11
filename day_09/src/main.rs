#[derive(Debug)]
enum Block {
    Gap(usize),
    File { id: usize, size: usize },
}

fn main() {
    let lines = std::fs::read_to_string("./input.txt")
        .expect("File should exist")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let input = lines.first().unwrap();

    let mut expanded = Vec::new();
    for (i, ch) in input.chars().enumerate() {
        if i % 2 == 0 {
            let size: usize = ch.to_digit(10).unwrap().try_into().unwrap();
            let id: usize = i / 2;
            expanded.push(Block::File { id, size });
        } else {
            let size: usize = ch.to_digit(10).unwrap().try_into().unwrap();
            if size > 0 {
                expanded.push(Block::Gap(size));
            }
        }
    }

    let len = expanded.len();
    let mut j = len - 1;
    loop {
        if let Block::File { id, size } = expanded[j] {
            match expanded.iter().enumerate().find(|(i, block)| match block {
                Block::Gap(gap_size) => *gap_size >= size && *i < j,
                Block::File { .. } => false,
            }) {
                Some((i, Block::Gap(gap_size))) => {
                    let diff = gap_size - size;
                    expanded[i] = Block::File { id, size };
                    expanded[j] = Block::Gap(size);
                    if diff > 0 {
                        expanded.insert(i + 1, Block::Gap(diff));
                        j += 1;
                    }
                }
                Some((_, Block::File { .. })) => unreachable!(),
                None => {}
            }
        }

        if j == 0 {
            break;
        } else {
            j -= 1;
        };
    }

    let mut x = String::new();
    for a in expanded.iter() {
        match a {
            Block::Gap(size) => {
                x.extend((0..*size).map(|_| "."));
            }
            Block::File { id, size } => {
                x.extend((0..*size).map(|_| id.to_string()));
            }
        }
    }

    dbg!(x);

    let (checksum, _) = expanded
        .iter()
        .fold((0, 0), |(total, idx), block| match block {
            Block::Gap(size) => (total, idx + size),
            Block::File { id, size } => (
                (total + (idx..idx + size).fold(0, |acc, x| acc + x * id)),
                idx + size,
            ),
        });

    dbg!(checksum);
}

//fn main() {
//    let lines = std::fs::read_to_string("./input.txt")
//        .expect("File should exist")
//        .lines()
//        .map(String::from)
//        .collect::<Vec<_>>();
//
//    let input = lines.first().unwrap();
//
//    let mut expanded = Vec::new();
//    for (i, ch) in input.chars().enumerate() {
//        if i % 2 == 0 {
//            let d: usize = ch.to_digit(10).unwrap().try_into().unwrap();
//            let id = (i / 2) + 48;
//            let ch = char::from_u32(id.try_into().unwrap()).unwrap();
//
//            let mut x = vec![ch; d];
//            expanded.append(&mut x);
//        } else {
//            let mut x = vec!['.'; ch.to_digit(10).unwrap().try_into().unwrap()];
//            expanded.append(&mut x);
//        }
//    }
//
//    let len = expanded.len();
//    for i in 0..len {
//        let l = expanded[i];
//        if l == '.' {
//            match expanded
//                .iter()
//                .rev()
//                .enumerate()
//                .find(|(j, r)| **r != '.' && (len - *j) > i)
//            {
//                Some((j, r)) => {
//                    expanded[i] = *r;
//                    expanded[len - j - 1] = l;
//                }
//                None => {
//                    break;
//                }
//            }
//        }
//    }
//
//    let checksum = expanded
//        .iter()
//        .enumerate()
//        .fold(0, |acc, (i, ch)| match ch {
//            '.' => acc,
//            _ => acc + ((*ch as u64) - 48) * i as u64,
//        });
//
//    dbg!(checksum);
//}
