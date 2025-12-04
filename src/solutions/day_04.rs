use grid::Grid;
use itertools::{Itertools, enumerate};

const TEST_INPUT: &str = include_str!("../../test_input/day4.txt");
const INPUT: &str = include_str!("../../input/day_04.txt");

// enum Directoin {
//     Up = (0, -1),
//     Down = (0, 1),
//     Left = (-1, 0),
//     Right = (1, 0),
//     ULeft = (-1, -1),
//     URight = (1, -1),
//     DLeft = (-1, 1),
//     DRight = (1, 1),
// }

pub fn solve() -> u32 {
    let direction = [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
    ];

    let l = INPUT.lines().collect_vec();
    let col_count = l.first().unwrap().chars().count();
    let vv = l
        .iter()
        .map(|l| l.chars().collect_vec())
        .flatten()
        .collect_vec();

    let g = Grid::from_vec(vv, col_count);
    // println!("grid: {:?}", g);

    let cnt = g
        .indexed_iter()
        .filter_map(|((row, col), c)| {
            if c.to_string() == "@" {
                let mut count = 0;
                for dir in direction {
                    if let Some(i) = g.get(row as i64 + dir.0, col as i64 + dir.1) {
                        if c == i {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    return Some(1);
                }
            }
            None
        })
        .count();

    return cnt.try_into().unwrap();
}

pub fn solve_2() -> u32 {
    let direction = [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
    ];

    let l = INPUT.lines().collect_vec();
    let col_count = l.first().unwrap().chars().count();
    let vv = l
        .iter()
        .map(|l| l.chars().collect_vec())
        .flatten()
        .collect_vec();

    let mut g = Grid::from_vec(vv, col_count);

    let mut amt = 0_usize;

    loop {
        let mut ind_remove = Vec::new();
        let cnt = g
            .indexed_iter()
            .filter_map(|((row, col), c)| {
                if c.to_string() == "@" {
                    let mut count = 0;
                    for dir in direction {
                        if let Some(i) = g.get(row as i64 + dir.0, col as i64 + dir.1) {
                            if c == i {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 {
                        ind_remove.push((row, col));
                        return Some(1);
                    }
                }
                None
            })
            .count();

        if cnt == 0 {
            return amt.try_into().unwrap();
        }
        for idx in ind_remove {
            g[idx] = '.';
        }
        // println!("{:?}", g);
        amt = amt + cnt;
    }
}
