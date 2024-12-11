use std::collections::HashMap;

fn blink(n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![1];
    }

    let s = n.to_string();
    if s.len() % 2 == 0 {
        let (l, r) = s.split_at(s.len() / 2);
        vec![l.parse().unwrap(), r.parse().unwrap()]
    } else {
        vec![n * 2024]
    }
}

fn solve(n: i64, initial: Vec<i64>) -> i64 {
    let mut current = initial;
    for _ in 0..n {
        current = current.into_iter().flat_map(blink).collect();
        dbg!(&current);
    }
    dbg!(&current);
    current.len() as i64
}

type Counter = HashMap<i64, i64>;

fn count(numbers: Vec<i64>) -> Counter {
    let mut counter = HashMap::new();
    for num in numbers {
        *counter.entry(num).or_insert(0) += 1;
    }
    counter
}

fn blink_with_count(pair: (i64, i64)) -> Vec<(i64, i64)> {
    blink(pair.0).into_iter().map(|x| (x, pair.1)).collect()
}

fn solve_with_counter(n: i64, initial: Vec<i64>) -> i64 {
    let mut current = count(initial);

    for _ in 0..n {
        dbg!(&current);
        current = current.into_iter().flat_map(blink_with_count).fold(
            HashMap::new(),
            |mut acc, (key, val)| {
                *acc.entry(key).or_insert(0) += val;
                acc
            },
        );
    }

    dbg!(&current);
    current.values().sum()
}

fn main() {
    let input = "0 89741 316108 7641 756 9 7832357 91";
    //let input = "125 17";

    let blink_count = 75;

    let result = input
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let res = solve_with_counter(blink_count, result);

    dbg!(res);
}
