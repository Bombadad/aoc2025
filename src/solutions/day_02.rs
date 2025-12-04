use itertools::Itertools;

const TEST_INPUT: &str = include_str!("../../test_input/day2.txt");
const INPUT: &str = include_str!("../../input/day_02.txt");

struct ProductIds {
    start: u64,
    end: u64,
}

impl IntoIterator for ProductIds {
    type Item = u64;
    type IntoIter = std::ops::RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { start, end } = self;
        start..=end
    }
}

pub fn solve() -> u64 {
    let ranges = INPUT
        .trim()
        .split(',')
        .map(|ss| {
            let temp = ss.split_once("-").unwrap();
            // println!("temp: {:?}", temp);
            ProductIds {
                start: temp.0.parse().unwrap(),
                end: temp.1.parse().unwrap(),
            }
        })
        .collect_vec();

    ranges
        .into_iter()
        .flat_map(|ids| ids.into_iter())
        .filter(|id| is_valid(*id))
        .sum()
}

pub fn solve_2() -> u64 {
    let ranges = INPUT
        .trim()
        .split(',')
        .map(|ss| {
            let temp = ss.split_once("-").unwrap();
            // println!("temp: {:?}", temp);
            ProductIds {
                start: temp.0.parse().unwrap(),
                end: temp.1.parse().unwrap(),
            }
        })
        .collect_vec();

    ranges
        .into_iter()
        .flat_map(|ids| ids.into_iter())
        .filter(|id| !is_valid_2(*id))
        .sum()
}

fn is_valid(id: u64) -> bool {
    let s = id.to_string();
    let half = s.len() / 2;

    if !s.is_char_boundary(half) {
        return false;
    }

    let (right, left) = s.split_at(half);
    if right.len() != left.len() || right != left {
        return false;
    }
    true
}

fn is_valid_2(id: u64) -> bool {
    let s = id.to_string();
    let s = s.as_bytes();
    for chunks_size in 1..=(s.len() / 2) {
        if !s.len().is_multiple_of(chunks_size) {
            continue;
        }

        if s.chunks(chunks_size).all_equal() {
            return false;
        }
    }

    true
}
