use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day3.txt");
const INPUT: &str = include_str!("../../input/day_03.txt");

pub fn solve() -> u32 {
    let input = INPUT
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let s = input
        .iter()
        .map(|l| {
            let l_temp = &l[..l.len() - 1];
            let (index, max_1) = l_temp
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, val)| **val)
                .unwrap();

            // println!("index: {}, max: {}", index, max_1);

            let max_2 = l[index + 1..].iter().max().unwrap();
            max_1 * 10 + max_2
        })
        .sum();

    return s;
}

pub fn solve_2() -> u64 {
    let input = INPUT
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect_vec()
        })
        .collect_vec();

    let s = input
        .iter()
        .map(|l| {
            let mut total = 0;
            let mut index = 0;
            let l_temp = l.iter().enumerate().collect_vec();
            for i in 0..12 {
                // let l_temp = &l[index..l.len() - 12 - i];
                let (idx, max_1) = l_temp[index..l.len() - 11 + i]
                    .iter()
                    .rev()
                    .max_by_key(|(_, val)| **val)
                    .unwrap();

                // println!("index: {}, max: {}", idx, max_1);

                index = *idx + 1;
                total = total * 10 + *max_1;

                // println!("index_new: {}, total: {}", index, total);
            }
            total
        })
        .sum();

    return s;
}
