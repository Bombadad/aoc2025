use grid::Grid;
use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day6.txt");
const INPUT: &str = include_str!("../../input/day_06.txt");

pub fn solve() -> u64 {
    let mut l = INPUT.lines().rev();
    let ops_line = l.next().unwrap();
    let ops = ops_line.trim().split_whitespace().collect_vec();

    let args_vec = l
        .map(|ln| {
            ln.trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let mut args_vec = Grid::from_vec(args_vec, ops.len());

    args_vec.transpose();
    let mut res = 0;
    for (i, op) in ops.iter().enumerate() {
        let problem = match *op {
            "+" => args_vec.iter_row(i).sum(),
            "*" => args_vec.iter_row(i).fold(1, |acc, n| acc * n),
            _ => unreachable!(),
        };
        res += problem
    }

    return res;
}
pub fn solve_2() -> u64 {
    let mut l = TEST_INPUT.lines();
    let ops_line = l.next_back().unwrap();
    let ops = ops_line.trim().split_whitespace().collect_vec();
    let num_lengths = ops_line.split(" ").collect_vec();
    let mut lengths = Vec::new();

    for c in num_lengths {
        if !c.is_empty() {
            lengths.push(0);
            continue;
        }
        if let Some(last) = lengths.last_mut() {
            *last = *last + 1;
        }
    }

    if let Some(last) = lengths.last_mut() {
        *last = *last + 1;
    }
    let args_vec = l
        .map(|ln| ln.trim().split_whitespace().collect_vec())
        .flatten()
        .collect_vec();

    let args_lengths = Vec::new();
    for line in l {
        let c = line.chars();
        for len in lengths {
            // args_lengths.drain(range);
        }
    }

    let mut args_vec = Grid::from_vec(args_vec, ops.len());

    args_vec.transpose();
    let mut res = 0;
    // for (i, op) in ops.iter().enumerate() {
    //     let nums = args_vec
    //         .iter_row(i)
    //         .map(|n| n.chars().collect_vec())
    //         .collect_vec();
    //     let problem = match *op {
    //         "+" => args_vec.iter_row(i).sum(),
    //         "*" => args_vec.iter_row(i).fold(1, |acc, n| acc * n),
    //         _ => unreachable!(),
    //     };
    //     res += problem
    // }

    return res;
}
