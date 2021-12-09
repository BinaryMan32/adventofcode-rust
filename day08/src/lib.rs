use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;

struct DigitSet {
    digits: Vec<String>
}

static SYMBOLS: &str = "abcdefg";

impl DigitSet {
    fn parse(input: &str) -> DigitSet {
        DigitSet {
            digits: input.split(" ").into_iter()
                .map(|x| x.chars().sorted().collect())
                .collect()
        }
    }

    fn unique_digits(&self) -> usize {
        self.digits.iter()
            .map(|d| d.len())
            .filter(|size| UNIQUE_SIZES.contains(size))
            .count()
    }

    fn digits_by_len(&self) -> HashMap<usize, Vec<HashSet<char>>> {
        self.digits.iter()
            .into_group_map_by(|d| d.len())
            .into_iter()
            .map(|(len, digits_group)|
                (len, digits_group
                    .into_iter()
                    .map(|digit| digit.chars()
                        .collect::<HashSet<char>>()
                    ).collect::<Vec<_>>()
                )
            )
            .collect::<HashMap<_, _>>()
    }

    fn segments_by_count(&self) -> HashMap<usize, Vec<char>> {
        SYMBOLS.chars()
            .into_group_map_by(|&s|
                self.digits.iter()
                    .map(|d| d.chars().filter(|&x| x==s).count())
                    .sum()
            )
            .into_iter()
            .collect()
    }

    fn get_digit_mapping(&self) -> HashMap<String, i64> {
        /*
         * For the 7 segment display as follows:
         *
         *   aaaa
         *  b    c
         *  b    c
         *   dddd
         *  e    f
         *  e    f
         *   gggg
         *
         * Mark the digits which occur uniquely, and the frequency of each segment:
         *
         *    abcdefg
         *  0 xxx xxx 6 
         *  1   x  x  2 x
         *  2 x xxx x 5
         *  3 x xx xx 5
         *  4  xxx x  4 x
         *  5 xx x xx 5
         *  6 xx xxxx 6
         *  7 x x  x  3 x
         *  8 xxxxxxx 7 x
         *  9 xxxx xx 6
         *    8787497
         *        xx
         */
        let digits_by_len = self.digits_by_len();
        let the1 = digits_by_len.get(&2).unwrap().first().unwrap();
        let the4 = digits_by_len.get(&4).unwrap().first().unwrap();
        let the7 = digits_by_len.get(&3).unwrap().first().unwrap();
        let the8 = digits_by_len.get(&7).unwrap().first().unwrap();

        let digits_with_5_segments = digits_by_len.get(&5).unwrap();
        let digits_with_6_segments = digits_by_len.get(&6).unwrap();

        let segments_by_count = self.segments_by_count();
        let segment_e_4_times = segments_by_count.get(&4).unwrap().first().unwrap();
        let segment_f_9_times = segments_by_count.get(&9).unwrap().first().unwrap();

        // 2 does not use segment f
        let the2 = digits_with_5_segments.iter().find(|d| !d.contains(segment_f_9_times)).unwrap();
        let the3or5 = digits_with_5_segments.iter().filter(|&x| x != the2).collect::<Vec<_>>();

        // 9 does not use segment e
        let the9 = digits_with_6_segments.iter().find(|d| !d.contains(segment_e_4_times)).unwrap();
        let the0or6 = digits_with_6_segments.iter().filter(|&x| x != the9).collect::<Vec<_>>();

        // segment c is the only one common between 1 and 2
        let segment_c = the1.intersection(the2).next().unwrap();

        // 3 has segment c and 5 does not
        let (the3, the5) = if the3or5[0].contains(segment_c) {(the3or5[0], the3or5[1])} else {(the3or5[1], the3or5[0])};

        // 0 has segment c and 6 does not
        let (the0, the6) = if the0or6[0].contains(segment_c) {(the0or6[0], the0or6[1])} else {(the0or6[1], the0or6[0])};

        [
            (the0, 0),
            (the1, 1),
            (the2, 2),
            (the3, 3),
            (the4, 4),
            (the5, 5),
            (the6, 6),
            (the7, 7),
            (the8, 8),
            (the9, 9)
        ].map(|(digit, num)| (digit.into_iter().sorted().collect(), num)).into_iter().collect()
    }

    fn decode(&self, mapping: HashMap<String, i64>) -> i64 {
        self.digits.iter().fold(0, |sum, digit| {
            sum * 10 + mapping.get(digit).unwrap()
        })
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
    input.iter()
        .map(|line| Entry::parse(line))
        .map(|entry| {
            let mapping = entry.patterns.get_digit_mapping();
            entry.digits.decode(mapping)
        })
        .sum()
}
