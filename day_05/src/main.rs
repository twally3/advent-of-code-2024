fn main() {
    let lines = std::fs::read_to_string("./input.txt")
        .expect("File should exist")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut is_dep = true;
    let mut deps = Vec::new();
    let mut pages_list = Vec::new();

    for line in lines {
        if line.is_empty() {
            is_dep = false;
            continue;
        }

        if is_dep {
            let [a, b]: [String; 2] = line
                .split("|")
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            deps.push((a, b));
        } else {
            let page_list = line.split(",").map(String::from).collect::<Vec<_>>();
            pages_list.push(page_list);
        }
    }

    let mut results = Vec::new();

    for pages in &pages_list {
        let mut seen_pages = std::collections::HashSet::new();
        let mut result = true;

        for page in pages {
            seen_pages.insert(page.clone());

            if !deps
                .iter()
                .filter_map(|(a, b)| if *a == *page { Some(b.clone()) } else { None })
                .collect::<std::collections::HashSet<_>>()
                .intersection(&seen_pages)
                .collect::<Vec<_>>()
                .is_empty()
            {
                result = false;
                break;
            }
        }

        results.push(result);
    }

    let total = results
        .iter()
        .zip(&pages_list)
        .filter_map(|(result, pages)| {
            if !*result {
                let mut pages = pages.clone();
                pages.sort_by(|x, y| {
                    match deps
                        .iter()
                        .find(|(a, b)| (a == x && b == y) || (b == x && a == y))
                    {
                        Some((a, _)) => {
                            if a == x {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        }
                        None => std::cmp::Ordering::Equal,
                    }
                });
                Some(pages[pages.len() / 2].clone())
            } else {
                None
            }
        })
        .fold(0, |acc, x| x.parse::<i32>().unwrap() + acc);

    dbg!(total);
}

//fn main() {
//    let lines = std::fs::read_to_string("./input.txt")
//        .expect("File should exist")
//        .lines()
//        .map(String::from)
//        .collect::<Vec<_>>();
//
//    let mut is_dep = true;
//    let mut deps = Vec::new();
//    let mut pages_list = Vec::new();
//
//    for line in lines {
//        if line.is_empty() {
//            is_dep = false;
//            continue;
//        }
//
//        if is_dep {
//            let [a, b]: [String; 2] = line
//                .split("|")
//                .map(|x| x.to_string())
//                .collect::<Vec<_>>()
//                .try_into()
//                .unwrap();
//
//            deps.push((a, b));
//        } else {
//            let page_list = line.split(",").map(String::from).collect::<Vec<_>>();
//            pages_list.push(page_list);
//        }
//    }
//
//    let mut results = Vec::new();
//
//    for pages in &pages_list {
//        let mut seen_pages = std::collections::HashSet::new();
//        let mut result = true;
//
//        for page in pages {
//            seen_pages.insert(page.clone());
//
//            if !deps
//                .iter()
//                .filter_map(|(a, b)| if *a == *page { Some(b.clone()) } else { None })
//                .collect::<std::collections::HashSet<_>>()
//                .intersection(&seen_pages)
//                .collect::<Vec<_>>()
//                .is_empty()
//            {
//                result = false;
//                break;
//            }
//        }
//
//        results.push(result);
//    }
//
//    let total = results
//        .iter()
//        .zip(&pages_list)
//        .filter_map(|(result, page_list)| {
//            if *result {
//                Some(page_list[page_list.len() / 2].clone())
//            } else {
//                None
//            }
//        })
//        .fold(0, |acc, x| x.parse::<i32>().unwrap() + acc);
//
//    dbg!(total);
//}
