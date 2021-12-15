use itertools::Itertools;
use std::collections::HashMap;
use std::iter;

struct Rule {
    pair: (char, char),
    insert: char
}

fn parse_rule(line: &str) -> Rule {
    println!("line:{}", line);
    let (pair_str, insert_str) = line.split(" -> ")
        .collect_tuple::<(&str, &str)>()
        .unwrap();
    Rule {
        pair: pair_str.chars().collect_tuple().unwrap(),
        insert: insert_str.chars().next().unwrap()
    }
}

fn parse_input(input: &Vec<&str>) -> (String, HashMap<(char, char), char>) {
    let rules = input[2..].iter()
        .map(|line| parse_rule(line))
        .map(|rule| (rule.pair, rule.insert))
        .collect();
    (input[0].to_string(), rules)
}

fn apply_rules(input: &str, rules: &HashMap<(char, char), char>) -> String {
    let chars = input.chars();
    let last = input.chars().last().unwrap();
    chars.tuple_windows()
        .flat_map(|(a, b)| match rules.get(&(a, b)) {
            Some(&insert) => vec![a, insert],
            None => vec![a]
        })
        .chain(iter::once(last))
        .collect()
}

fn char_counts(input: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    input.chars().for_each(|c| *counts.entry(c).or_insert(0) += 1);
    counts
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let (template, rules) = parse_input(input);
    let result = std::iter::successors(Some(template), |input| Some(apply_rules(input, &rules)))
        .skip(10)
        .next()
        .unwrap();
    let counts = char_counts(&result);
    let num_least_common = counts.values().min().unwrap();
    let num_most_common = counts.values().max().unwrap();
    (num_most_common - num_least_common) as i64
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
