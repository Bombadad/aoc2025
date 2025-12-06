use std::collections::BTreeSet;

use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day5.txt");
const INPUT: &str = include_str!("../../input/day_05.txt");

pub fn solve() -> u64 {
    let (id_ranges, aval_ids) = INPUT.trim().split_once("\n\n").unwrap();

    // println!("id_r: {}, a_id: {}", id_ranges, aval_ids);

    // let mut bt_set: BTreeSet<(u64, u64)> = BTreeSet::new();
    let mut bt_set: Vec<(u64, u64)> = Vec::new();
    let mut i = 0;

    for l in id_ranges.lines() {
        let (start, end) = l.split_once("-").unwrap();
        // let mut temp_bt: BTreeSet<u64> = (start.parse().unwrap()..=end.parse().unwrap()).collect();
        // bt_set.append(&mut temp_bt);
        bt_set.push((start.parse().unwrap(), end.parse().unwrap()));
        i += 1;
    }

    let cnt = aval_ids
        .lines()
        .filter_map(|id| {
            let food_id = id.parse::<u64>().unwrap();
            for (start, end) in bt_set.iter() {
                if food_id >= *start && food_id <= *end {
                    return Some(food_id);
                }
            }
            None
        })
        .count();

    return cnt.try_into().unwrap();
}

pub fn solve_2() -> u64 {
    let (id_ranges, _) = INPUT.trim().split_once("\n\n").unwrap();

    // println!("id_r: {}, a_id: {}", id_ranges, aval_ids);

    // let mut bt_set: BTreeSet<(u64, u64)> = BTreeSet::new();
    let mut bt_set: Vec<(u64, u64)> = Vec::new();

    for l in id_ranges.lines() {
        let (start, end) = l.split_once("-").unwrap();
        // let mut temp_bt: BTreeSet<u64> = (start.parse().unwrap()..=end.parse().unwrap()).collect();
        // bt_set.append(&mut temp_bt);
        bt_set.push((start.parse().unwrap(), end.parse().unwrap()));
    }

    let new_vec = merge(bt_set);

    let s: u64 = new_vec.iter().map(|(start, end)| (end - start) + 1).sum();

    return s;
}

pub fn merge(input: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut input = input.iter().sorted_unstable_by_key(|range| range.0);
    let mut res: Vec<(u64, u64)> = Vec::new();
    res.push(*input.next().unwrap());

    for i in input {
        let last = res.last_mut().unwrap();

        if last.1 < i.0 {
            res.push(*i);
            continue;
        }
        *last = (last.0, last.1.max(i.1));
    }

    return res;
}
