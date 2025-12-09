use std::cmp::Ordering;

use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day8.txt");
const INPUT: &str = include_str!("../../input/day_08.txt");

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    pub fn distance(&self, other: &Self) -> i64 {
        let dx = (self.x as i64) - (other.x as i64);
        let dy = (self.y as i64) - (other.y as i64);
        let dz = (self.z as i64) - (other.z as i64);
        dx.pow(2) + dy.pow(2) + dz.pow(2)
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
    let lines = TEST_INPUT.lines();

    return 0;
}
