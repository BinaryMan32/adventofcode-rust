fn parse_positions(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn min_cost(positions: &Vec<i64>, cost_fn: fn (i64, i64) -> i64) -> i64 {
    let max_position = positions.iter().max().unwrap();
    (0..(max_position+1))
        .map(|p| positions.iter().map(|&x| cost_fn(x, p)).sum())
        .min()
        .unwrap()
}

fn cost1(a: i64, b: i64) -> i64 {
    (a - b).abs()
}

pub fn part1(input: &Vec<&str>) -> i64 {
    min_cost(&parse_positions(input[0]), cost1)
}

fn cost2(a: i64, b: i64) -> i64 {
    let n = (a - b).abs();
    n * (n + 1) / 2
}

pub fn part2(input: &Vec<&str>) -> i64 {
    min_cost(&parse_positions(input[0]), cost2)
}
