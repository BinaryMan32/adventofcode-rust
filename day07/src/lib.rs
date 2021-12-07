fn parse_positions(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let positions = parse_positions(input[0]);
    let max_position = positions.iter().max().unwrap();
    (0..(max_position+1))
        .map(|p| positions.iter().map(|x| (x-p).abs()).sum())
        .min()
        .unwrap()
}

fn cost(a: i64, b: i64) -> i64 {
    let n = (a - b).abs();
    n * (n + 1) / 2
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let positions = parse_positions(input[0]);
    let max_position = positions.iter().max().unwrap();
    (0..(max_position+1))
        .map(|p| positions.iter().map(|&x| cost(x, p)).sum())
        .min()
        .unwrap()
}
