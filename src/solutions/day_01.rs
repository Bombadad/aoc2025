use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day1.txt");
const INPUT: &str = include_str!("../../input/day_01.txt");

pub fn solve() -> u64 {
    // let l = TEST_INPUT.lines();
    let l = INPUT.lines();
    let dir_dis = l
        .map(|l| {
            let splits = l.split_at(1);
            (splits.0, splits.1.parse::<i64>().unwrap())
        })
        .collect_vec();
    let mut loc: i64 = 50;
    let cnt = dir_dis.iter().filter_map(|(dir, dis)| {
        let turns = dis % 100;
        loc = match *dir {
            "L" => (loc - turns + 100) % 100,
            "R" => (loc + turns) % 100,
            _ => unreachable!(),
        };
        // println!("{}", loc);
        if loc != 0 { None } else { Some(loc) }
    });
    return cnt.count().try_into().unwrap();
}
pub fn solve_2() -> u64 {
    // let l = TEST_INPUT.lines();
    let l = INPUT.lines();
    let dir_dis = l
        .map(|l| {
            let splits = l.split_at(1);
            (splits.0, splits.1.parse::<i64>().unwrap())
        })
        .collect_vec();
    let mut loc: i64 = 50;
    let cnt = dir_dis
        .iter()
        .map(|(dir, dis)| {
            let turns = dis % 100;
            let mut times_zero = 0;
            loc = match *dir {
                "L" => {
                    let num = loc - dis;
                    if num <= 0 && loc != 0 {
                        times_zero += 1;
                    }
                    times_zero += (num / 100).abs();
                    (loc - turns + 100) % 100
                }
                "R" => {
                    let num = loc + dis;
                    times_zero = num / 100;
                    (loc + turns) % 100
                }
                _ => unreachable!(),
            };
            times_zero as u64
        })
        .collect_vec();
    return cnt.iter().sum();
}
