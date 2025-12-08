use std::collections::VecDeque;

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
    let mut l = INPUT.lines();
    let ops_line = l.next_back().unwrap();
    let ops = ops_line.trim().split_whitespace().collect_vec();
    let num_lengths = ops_line.split(" ").collect_vec();
    let mut lengths: Vec<u64> = Vec::new();

    for c in num_lengths {
        if !c.is_empty() {
            lengths.push(1);
            continue;
        }
        if let Some(last) = lengths.last_mut() {
            *last = *last + 1;
        }
    }

    // dbg!(&lengths);

    let mut args_lengths = vec![Vec::new(); lengths.len()];
    for line in l {
        let mut c = line.chars().collect::<VecDeque<_>>();
        for (i, len) in lengths.iter().enumerate() {
            let line_drain = c.drain(..*len as usize).collect_vec();
            // dbg!(line_drain);
            c.pop_front();
            args_lengths[i].push(line_drain);
        }
    }

    // dbg!(args_lengths);

    let mut res = 0;
    for (op, arg_vec) in ops.iter().zip(args_lengths.iter()) {
        let mut nums = vec![0; arg_vec.first().unwrap().len()];
        // dbg!(arg_vec);
        for arg in arg_vec {
            for (j, a) in arg.iter().enumerate() {
                if a.is_alphanumeric() {
                    nums[j] = nums[j] * 10 + (a.to_digit(10).unwrap() as u64);
                }
            }
        }

        //     let nums = args_vec
        //         .iter_row(i)
        //         .map(|n| n.chars().collect_vec())
        //         .collect_vec();
        let problem = match *op {
            "+" => nums.iter().sum(),
            "*" => nums.iter().fold(1, |acc, n| acc * n),
            _ => unreachable!(),
        };
        res += problem
    }

    return res;
}
