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
        Status::Incomplete(expected.iter().collect::<String>())
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    input.iter()
        .map(|line| match check_chunk(line) {
            Status::Corrupt(c) => corrupt_score(c),
            _ => 0
        })
        .sum()
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
