use std::collections::HashSet;

use lazy_static::lazy_static;

struct DigitSet {
    digits: Vec<HashSet<char>>
}

impl DigitSet {
    fn parse(input: &str) -> DigitSet {
        DigitSet {
            digits: input.split(" ").into_iter()
                .map(|x| x.chars().collect::<HashSet<char>>())
                .collect()
        }
    }

    fn unique_digits(&self) -> usize {
        self.digits.iter()
            .map(|d| d.len())
            .filter(|size| UNIQUE_SIZES.contains(size))
            .count()
    }
}

struct Entry {
    patterns: DigitSet,
    digits: DigitSet,
}

lazy_static! {
    // unique number of segments for digits 1, 4, 7, 8
    static ref UNIQUE_SIZES: HashSet<usize> = HashSet::from_iter([2, 4, 3, 7]);
}

impl Entry {
    fn parse(line: &str) -> Entry {
        let parts = line.split(" | ").into_iter().collect::<Vec<_>>();
        Entry {
            patterns: DigitSet::parse(parts[0]),
            digits: DigitSet::parse(parts[1]),
        }
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    input.iter()
        .map(|line| Entry::parse(line))
        .map(|entry| entry.digits.unique_digits() as i64)
        .sum()
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
