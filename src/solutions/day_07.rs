use std::collections::{HashSet, VecDeque};

use grid::Grid;
use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day7.txt");
const INPUT: &str = include_str!("../../input/day_07.txt");

pub fn solve() -> u64 {
    let mut lines = INPUT.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut tach_q = VecDeque::new();
    let mut split_cnt = 0;

    // dbg!(&lines);

    tach_q.push_back((
        0_usize,
        lines[0].iter().find_position(|c| **c == 'S').unwrap().0,
    ));

    while !tach_q.is_empty() {
        if let Some(current_pos) = tach_q.pop_back() {
            // dbg!(&current_pos);
            // println!("{:?}", current_pos);
            if let Some(char_test) = lines
                .get(current_pos.0 + 1)
                .and_then(|row| row.get(current_pos.1))
            {
                match char_test {
                    '|' => {
                        continue;
                    }
                    '.' => {
                        tach_q.push_front((current_pos.0 + 1, current_pos.1));
                        lines[current_pos.0 + 1][current_pos.1] = '|';
                    }
                    '^' => {
                        if let Some(tach_check) = lines
                            .get(current_pos.0 + 1)
                            .and_then(|row| row.get(current_pos.1 + 1))
                        {
                            if *tach_check != '|' {
                                tach_q.push_front((current_pos.0 + 1, current_pos.1 + 1));
                                lines[current_pos.0 + 1][current_pos.1 + 1] = '|';
                            }
                        }
                        if let Some(tach_check) = lines
                            .get(current_pos.0 + 1)
                            .and_then(|row| row.get(current_pos.1 - 1))
                        {
                            if *tach_check != '|' {
                                tach_q.push_front((current_pos.0 + 1, current_pos.1 - 1));
                                lines[current_pos.0 + 1][current_pos.1 - 1] = '|';
                            }
                        }
                        split_cnt += 1;
                    }
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    return split_cnt;
}

pub fn solve_2() -> u64 {
    let mut lines = INPUT.lines();
    let first_ln = lines.next().unwrap();
    let mut row = vec![1; first_ln.len()];

    // dbg!(&lines);

    for line in INPUT.lines().rev() {
        for (i, c) in line.chars().enumerate() {
            if c == '^' {
                row[i] = row[i - 1] + row[i + 1];
            }
        }
    }
    return row[first_ln.find('S').unwrap()];
}
