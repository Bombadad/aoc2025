use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day9.txt");
const INPUT: &str = include_str!("../../input/day_09.txt");

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn area(&self, other: &Self) -> i64 {
        ((self.x as i64 - other.x as i64) + 1).abs() * ((self.y as i64 - other.y as i64) + 1).abs()
    }
}

struct Edge {
    first: Point,
    end: Point,
}

pub fn solve() -> u64 {
    let mut areas = INPUT
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();

            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .tuple_combinations()
        .map(|(a, b)| a.area(&b))
        .collect_vec();

    areas.sort();
    // println!("{:?}", areas);
    return areas.last().unwrap().to_owned().try_into().unwrap();
}

pub fn solve_2() -> u64 {
    let mut areas = TEST_INPUT
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();

            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect_vec();

    // let mut

    // lets make the edges
    // println!("{:?}", areas);
    return 0;
}
