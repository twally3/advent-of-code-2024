fn main() {
    let lines = std::fs::read_to_string("input.txt")
        .expect("File to be loaded")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in lines {
        let [a, b]: [u32; 2] = line
            .split("   ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        left_list.push(a);
        right_list.push(b);
    }

    let mut similarity_score = 0;
    for a in left_list {
        let mut count = 0;
        for b in &right_list {
            if a == *b {
                count += 1;
            }
        }
        similarity_score += a * count;
    }
    println!("{similarity_score}");

    //left_list.sort();
    //right_list.sort();
    //
    //let total_distance = left_list
    //    .iter()
    //    .zip(right_list)
    //    .map(|(a, b)| a.abs_diff(b))
    //    .sum::<u32>();
    //
    //println!("{total_distance}");
}
