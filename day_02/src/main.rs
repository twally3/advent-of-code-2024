#[derive(Debug, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

fn main() {
    let lines = std::fs::read_to_string("sample.txt")
        .expect("File to be loaded")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut count = 0;
    for line in lines {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let permutations = (0..levels.len())
            .map(|i| {
                levels
                    .iter()
                    .enumerate()
                    .filter(move |(j, _)| *j != i)
                    .map(|(_, x)| *x)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut safety = Safety::Safe;

        for permutation in permutations {
            let mut last_direction = std::cmp::Ordering::Equal;

            for (i, level) in permutation.windows(2).enumerate() {
                let [a, b] = level else { unreachable!() };

                let dir = a.cmp(b);
                let diff = a.abs_diff(*b);

                if last_direction != dir && i != 0 || !(1..=3).contains(&diff) {
                    safety = Safety::Unsafe;
                    break;
                } else {
                    last_direction = dir;
                    safety = Safety::Safe;
                };
            }

            if safety == Safety::Safe {
                break;
            }
        }

        if safety == Safety::Safe {
            count += 1;
        }
    }

    println!("{count}");
}

//fn main() {
//    let lines = std::fs::read_to_string("input.txt")
//        .expect("File to be loaded")
//        .lines()
//        .map(String::from)
//        .collect::<Vec<_>>();
//
//    let mut count = 0;
//    for line in lines {
//        let mut last_direction = std::cmp::Ordering::Equal;
//        let mut safety = Safety::Safe;
//
//        for (i, level) in line
//            .split_whitespace()
//            .map(|x| x.parse::<i32>().unwrap())
//            .collect::<Vec<_>>()
//            .as_slice()
//            .windows(2)
//            .enumerate()
//        {
//            let [a, b] = level else { unreachable!() };
//
//            let dir = a.cmp(b);
//            let diff = a.abs_diff(*b);
//
//            if last_direction != dir && i != 0 || !(1..=3).contains(&diff) {
//                safety = Safety::Unsafe;
//                break;
//            } else {
//                last_direction = dir;
//                safety = Safety::Safe;
//            };
//        }
//
//        if safety == Safety::Safe {
//            count += 1;
//        }
//    }
//
//    println!("{count}");
//}
