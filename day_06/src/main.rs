use std::{
    sync::{atomic::AtomicUsize, Arc},
    thread,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Obstacle,
    Space,
    Guard(Direction),
    Obstruction,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Obstacle => "#",
                Cell::Space => ".",
                Cell::Guard(dir) => match dir {
                    Direction::Right => ">",
                    Direction::Left => "<",
                    Direction::Up => "^",
                    Direction::Down => "v",
                },
                Cell::Obstruction => "O",
            }
        )
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Obstacle),
            '.' => Ok(Cell::Space),
            '^' => Ok(Cell::Guard(Direction::Up)),
            '<' => Ok(Cell::Guard(Direction::Left)),
            '>' => Ok(Cell::Guard(Direction::Right)),
            'v' => Ok(Cell::Guard(Direction::Down)),
            _ => Err("FUCK".to_owned()),
        }
    }
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        let x = row
            .iter()
            .map(Cell::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        println!("{x}");
    }
    println!();
}

fn main() {
    let lines = std::fs::read_to_string("./input.txt")
        .expect("File should exist")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut base_grid = Vec::new();
    for line in lines {
        let mut cells = Vec::new();
        for ch in line.chars() {
            let cell = Cell::try_from(ch).unwrap();
            cells.push(cell);
        }
        base_grid.push(cells);
    }

    //let mut total = 0;
    //let original_grid = Arc::new(base_grid.clone());
    let original_grid = base_grid.clone();
    let total = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    let mut coordinates = Vec::new();
    for x in 0..original_grid.len() {
        for y in 0..original_grid[0].len() {
            coordinates.push((x, y));
        }
    }

    let threads = 16;
    let num_batches = coordinates.len() / threads;
    let batches = coordinates
        .chunks(num_batches)
        .map(Vec::from)
        .collect::<Vec<_>>();

    for (i, batch) in batches.into_iter().enumerate() {
        let original_grid = original_grid.clone();
        let total_clone = Arc::clone(&total);

        let handle = thread::spawn(move || {
            for (j, (x, y)) in batch.into_iter().enumerate() {
                if j % 10 == 0 {
                    dbg!(i, j);
                }
                if !matches!(original_grid[x][y], Cell::Space) {
                    continue;
                }
                let mut grid = original_grid.clone();
                grid[x][y] = Cell::Obstruction;

                let mut positions = std::collections::HashSet::new();

                'outer: loop {
                    let mut new_grid = vec![vec![Cell::Space; grid[0].len()]; grid.len()];

                    for (i, row) in grid.iter().enumerate() {
                        for (j, cell) in row.iter().enumerate() {
                            match cell {
                                Cell::Space => {
                                    if !matches!(new_grid[i][j], Cell::Guard(_)) {
                                        new_grid[i][j] = Cell::Space;
                                    }
                                }
                                Cell::Obstacle => {
                                    new_grid[i][j] = Cell::Obstacle;
                                }
                                Cell::Guard(dir) => {
                                    let tup = (dir.clone(), i, j);
                                    if positions.contains(&tup) {
                                        //total += 1;
                                        total_clone
                                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                        break 'outer;
                                    }

                                    positions.insert(tup);
                                    match dir {
                                        Direction::Up => {
                                            if i == 0 {
                                                break 'outer;
                                            }

                                            match &grid[i - 1][j] {
                                                Cell::Obstacle | Cell::Obstruction => {
                                                    new_grid[i][j] = Cell::Guard(Direction::Right);
                                                }
                                                Cell::Space => {
                                                    new_grid[i - 1][j] = Cell::Guard(Direction::Up);
                                                    new_grid[i][j] = Cell::Space;
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        Direction::Down => {
                                            if i == (grid.len() - 1) {
                                                break 'outer;
                                            }

                                            match &grid[i + 1][j] {
                                                Cell::Obstacle | Cell::Obstruction => {
                                                    new_grid[i][j] = Cell::Guard(Direction::Left);
                                                }
                                                Cell::Space => {
                                                    new_grid[i + 1][j] =
                                                        Cell::Guard(Direction::Down);
                                                    new_grid[i][j] = Cell::Space;
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        Direction::Left => {
                                            if j == 0 {
                                                break 'outer;
                                            }

                                            match &grid[i][j - 1] {
                                                Cell::Obstacle | Cell::Obstruction => {
                                                    new_grid[i][j] = Cell::Guard(Direction::Up);
                                                }
                                                Cell::Space => {
                                                    new_grid[i][j - 1] =
                                                        Cell::Guard(Direction::Left);
                                                    new_grid[i][j] = Cell::Space;
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        Direction::Right => {
                                            if j == row.len() - 1 {
                                                break 'outer;
                                            }

                                            match &grid[i][j + 1] {
                                                Cell::Obstacle | Cell::Obstruction => {
                                                    new_grid[i][j] = Cell::Guard(Direction::Down);
                                                }
                                                Cell::Space => {
                                                    new_grid[i][j + 1] =
                                                        Cell::Guard(Direction::Right);
                                                    new_grid[i][j] = Cell::Space;
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                }
                                Cell::Obstruction => {
                                    new_grid[i][j] = Cell::Obstruction;
                                }
                            }
                        }
                    }
                    grid = new_grid.clone();
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    dbg!(total.load(std::sync::atomic::Ordering::Relaxed));
}

//fn main() {
//    let lines = std::fs::read_to_string("./input.txt")
//        .expect("File should exist")
//        .lines()
//        .map(String::from)
//        .collect::<Vec<_>>();
//
//    let mut grid = Vec::new();
//    for line in lines {
//        let mut cells = Vec::new();
//        for ch in line.chars() {
//            let cell = Cell::try_from(ch).unwrap();
//            cells.push(cell);
//        }
//        grid.push(cells);
//    }
//
//    let mut positions = std::collections::HashSet::new();
//
//    'outer: loop {
//        //print_grid(&grid);
//        let mut new_grid = vec![vec![Cell::Space; grid[0].len()]; grid.len()];
//
//        for (i, row) in grid.iter().enumerate() {
//            for (j, cell) in row.iter().enumerate() {
//                match cell {
//                    Cell::Space => {
//                        if !matches!(new_grid[i][j], Cell::Guard(_)) {
//                            new_grid[i][j] = Cell::Space;
//                        }
//                    }
//                    Cell::Obstacle => {
//                        new_grid[i][j] = Cell::Obstacle;
//                    }
//                    Cell::Guard(dir) => match dir {
//                        Direction::Up => {
//                            positions.insert((i, j));
//                            if i == 0 {
//                                break 'outer;
//                            }
//
//                            match &grid[i - 1][j] {
//                                Cell::Obstacle => {
//                                    new_grid[i][j] = Cell::Guard(Direction::Right);
//                                }
//                                Cell::Space => {
//                                    new_grid[i - 1][j] = Cell::Guard(Direction::Up);
//                                    new_grid[i][j] = Cell::Space;
//                                }
//                                _ => unreachable!(),
//                            }
//                        }
//                        Direction::Down => {
//                            positions.insert((i, j));
//                            if i == (grid.len() - 1) {
//                                break 'outer;
//                            }
//
//                            match &grid[i + 1][j] {
//                                Cell::Obstacle => {
//                                    new_grid[i][j] = Cell::Guard(Direction::Left);
//                                }
//                                Cell::Space => {
//                                    new_grid[i + 1][j] = Cell::Guard(Direction::Down);
//                                    new_grid[i][j] = Cell::Space;
//                                }
//                                _ => unreachable!(),
//                            }
//                        }
//                        Direction::Left => {
//                            positions.insert((i, j));
//                            if j == 0 {
//                                break 'outer;
//                            }
//
//                            match &grid[i][j - 1] {
//                                Cell::Obstacle => {
//                                    new_grid[i][j] = Cell::Guard(Direction::Up);
//                                }
//                                Cell::Space => {
//                                    new_grid[i][j - 1] = Cell::Guard(Direction::Left);
//                                    new_grid[i][j] = Cell::Space;
//                                }
//                                _ => unreachable!(),
//                            }
//                        }
//                        Direction::Right => {
//                            positions.insert((i, j));
//                            if j == row.len() - 1 {
//                                break 'outer;
//                            }
//
//                            match &grid[i][j + 1] {
//                                Cell::Obstacle => {
//                                    new_grid[i][j] = Cell::Guard(Direction::Down);
//                                }
//                                Cell::Space => {
//                                    new_grid[i][j + 1] = Cell::Guard(Direction::Right);
//                                    new_grid[i][j] = Cell::Space;
//                                }
//                                _ => unreachable!(),
//                            }
//                        }
//                    },
//                }
//            }
//        }
//        grid = new_grid;
//    }
//
//    print_grid(&grid);
//
//    dbg!(positions.len());
//}
