enum Status {
    Ok,
    Incomplete(String),
    Corrupt(char)
}

fn corrupt_score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn check_chunk(line: &str) -> Status {
    let mut expected = Vec::<char>::new();
    for c in line.chars() {
        match c {
            '(' => expected.push(')'),
            '[' => expected.push(']'),
            '{' => expected.push('}'),
            '<' => expected.push('>'),
            _ if expected.last() == Some(&c) => {expected.pop();},
            _ => return Status::Corrupt(c)
        }
    }
    if expected.is_empty() {
        Status::Ok
    } else {
        Status::Incomplete(expected.iter().rev().collect::<String>())
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    input.iter()
        .filter_map(|line| match check_chunk(line) {
            Status::Corrupt(c) => Some(corrupt_score(c)),
            _ => None
        })
        .sum()
}

fn incomplete_score(incomplete: &str) -> i64 {
    incomplete.chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        })
        .fold(0, |total, c| total * 5 + c)
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let mut incomplete_scores = input.iter()
        .filter_map(|line| match check_chunk(line) {
            Status::Incomplete(expected) => Some(incomplete_score(&expected)),
            _ => None
        })
        .collect::<Vec<_>>();
    incomplete_scores.sort();
    incomplete_scores[incomplete_scores.len() / 2]
}
