use std::collections::HashSet;

use lazy_static::lazy_static;

struct Entry {
    patterns: Vec<HashSet<char>>,
    digits: Vec<HashSet<char>>,
}

lazy_static! {
    // unique number of segments for digits 1, 4, 7, 8
    static ref UNIQUE_SIZES: HashSet<usize> = HashSet::from_iter([2, 4, 3, 7]);
}

impl Entry {
    fn parse(line: &str) -> Entry {
        let parts = line.split(" | ").into_iter().collect::<Vec<_>>();
        Entry {
            patterns: parts[0].split(" ").into_iter()
                .map(|x| x.chars().collect::<HashSet<char>>())
                .collect(),
            digits: parts[1].split(" ").into_iter()
                .map(|x| x.chars().collect::<HashSet<char>>())
                .collect(),
        }
    }

    fn unique_digits(&self) -> usize {
        self.digits.iter()
            .map(|d| d.len())
            .filter(|size| UNIQUE_SIZES.contains(size))
            .count()
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    input.iter()
        .map(|line| Entry::parse(line))
        .inspect(|entry| println!("{:?} ({})", entry.digits, entry.unique_digits()))
        .map(|entry| entry.unique_digits() as i64)
        .sum()
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
