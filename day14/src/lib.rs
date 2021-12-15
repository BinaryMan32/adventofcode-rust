use itertools::Itertools;
use std::collections::HashMap;

struct Rule {
    pair: (char, char),
    insert: char
}

fn chars_to_string(a: char, b: char) -> String {
    [a, b].iter().collect()
}

impl Rule {
    fn from(&self) -> String {
        chars_to_string(self.pair.0, self.pair.1)
    }
    fn next(&self) -> [String; 2] {
        [
            chars_to_string(self.pair.0, self.insert),
            chars_to_string(self.insert, self.pair.1)
        ]
    }
}

fn parse_rule(line: &str) -> Rule {
    let (pair_str, insert_str) = line.split(" -> ")
        .collect_tuple::<(&str, &str)>()
        .unwrap();
    Rule {
        pair: pair_str.chars().collect_tuple().unwrap(),
        insert: insert_str.chars().next().unwrap()
    }
}

fn parse_input(input: &Vec<&str>) -> (String, HashMap<String, [String; 2]>) {
    let rules = input[2..].iter()
        .map(|line| parse_rule(line))
        .map(|rule| (rule.from(), rule.next()))
        .collect();
    (input[0].to_string(), rules)
}

fn apply_rules(pair_counts: &HashMap<String, usize>, rules: &HashMap<String, [String; 2]>) -> HashMap<String, usize> {
    let mut new_counts = HashMap::new();
    for (pair, count) in pair_counts {
        for next in rules.get(pair).unwrap() {
            *new_counts.entry(next.clone()).or_insert(0) += count
        }
    }
    new_counts
}

fn char_pairs(input: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    input.chars()
        .tuple_windows()
        .for_each(|(a, b)| *counts.entry(chars_to_string(a, b)).or_insert(0) += 1);
    counts
}

fn char_counts(template: &str, pairs: &HashMap<String, usize>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    pairs.iter().for_each(|(pair, count)| *counts.entry(pair.chars().next().unwrap()).or_insert(0) += count);
    *counts.entry(template.chars().last().unwrap()).or_insert(0) += 1;
    counts
}

fn iterate_steps(input: &Vec<&str>, steps: usize) -> i64 {
    let (template, rules) = parse_input(input);
    let result = std::iter::successors(Some(char_pairs(&template)), |input| Some(apply_rules(input, &rules)))
        .skip(steps)
        .next()
        .unwrap();
    let counts = char_counts(&template, &result);
    let num_least_common = counts.values().min().unwrap();
    let num_most_common = counts.values().max().unwrap();
    (num_most_common - num_least_common) as i64
}

pub fn part1(input: &Vec<&str>) -> i64 {
    iterate_steps(input, 10)
}

pub fn part2(input: &Vec<&str>) -> i64 {
    iterate_steps(input, 40)
}
