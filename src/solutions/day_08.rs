use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day8.txt");
const INPUT: &str = include_str!("../../input/day_08.txt");

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    pub fn distance(&self, other: &Self) -> i64 {
        (other.x as i64 - self.x as i64).pow(2)
            + (other.y as i64 - self.y as i64).pow(2)
            + (other.z as i64 - self.z as i64).pow(2)
    }
}

#[derive(Eq)]
struct PointPair {
    pub start: Point,
    pub end: Point,
    dist: i64,
}

impl PointPair {
    fn new(start: Point, end: Point) -> PointPair {
        PointPair {
            start,
            end,
            dist: start.distance(&end),
        }
    }
}

impl PartialEq for PointPair {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist).then_with(|| {
            self.start
                .cmp(&other.start)
                .then_with(|| self.end.cmp(&other.end))
        })
    }
}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() -> u64 {
    let lines = INPUT.lines();
    // let min_heap = BinaryHeap::new();
    let mut nodes = Vec::new();
    for line in lines {
        let mut split = line.split(",");
        let point = Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        };
        nodes.push(point);
    }

    let mut cirts = HashMap::new();
    let mut cirtc_cnt = vec![0; 500];

    let mut current = 0;

    nodes
        .iter()
        .tuple_combinations()
        .sorted_unstable_by(|(a, b), (c, d)| a.distance(b).cmp(&c.distance(d)))
        // .take(10)
        .take(1000)
        .for_each(|(start, end)| {
            // println!("start: {:?}, end: {:?}", start, end);
            if cirts.contains_key(start) && cirts.contains_key(end) {
                let cirt_num_start = cirts[start];
                let cirt_num_end = cirts[end];

                if cirt_num_start == cirt_num_end {
                    return;
                }

                for (_, value) in cirts.iter_mut() {
                    if *value == cirt_num_end {
                        *value = cirt_num_start;
                    }
                }

                cirtc_cnt[cirt_num_start] += cirtc_cnt[cirt_num_end];
                cirtc_cnt[cirt_num_end] = 0;
                return;
            }

            let circt_num = if cirts.contains_key(start) {
                cirts[start]
            } else if cirts.contains_key(end) {
                cirts[end]
            } else {
                current += 1;
                current
            };

            if !cirts.contains_key(start) {
                cirts.insert(start, circt_num);
                cirtc_cnt[circt_num] += 1;
            }
            if !cirts.contains_key(end) {
                cirts.insert(end, circt_num);
                cirtc_cnt[circt_num] += 1;
            }
            // println!("{:?}", cirts);
        });
    cirtc_cnt.sort();
    cirtc_cnt.iter().rev().take(3).product()
}
pub fn solve_2() -> u64 {
    let lines = INPUT.lines();
    // let min_heap = BinaryHeap::new();
    let mut nodes = Vec::new();
    for line in lines {
        let mut split = line.split(",");
        let point = Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        };
        nodes.push(point);
    }

    let mut cirts = HashMap::new();
    let mut cirtc_cnt = 0;

    let mut current = 0;

    let nodes = nodes
        .iter()
        .tuple_combinations()
        .sorted_unstable_by(|(a, b), (c, d)| a.distance(b).cmp(&c.distance(d)));

    for (start, end) in nodes {
        // println!("start: {:?}, end: {:?}", start, end);
        if cirts.contains_key(start) && cirts.contains_key(end) {
            let cirt_num_start = cirts[start];
            let cirt_num_end = cirts[end];

            if cirt_num_start == cirt_num_end {
                continue;
            }

            for (_, value) in cirts.iter_mut() {
                if *value == cirt_num_end {
                    *value = cirt_num_start;
                }
            }

            //cirts have combined
            cirtc_cnt -= 1;
            continue;
        }

        let circt_num = if cirts.contains_key(start) {
            cirts[start]
        } else if cirts.contains_key(end) {
            cirts[end]
        } else {
            cirtc_cnt += 1;
            current += 1;
            current
        };

        cirts.entry(start).or_insert(circt_num);
        cirts.entry(end).or_insert(circt_num);

        if cirts.len() == 1000 && cirtc_cnt == 1 {
            return start.x as u64 * end.x as u64;
        }
    }
    return 0;
}
