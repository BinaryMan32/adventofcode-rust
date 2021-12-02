pub fn part1(input: &Vec<i64>) -> usize {
    input.iter()
        .zip(&input[1..])
        .filter(|(&prev, &next)| next > prev)
        .count()
}

pub fn part2(input: &Vec<i64>) -> usize {
    input.iter()
        .zip(&input[3..])
        .filter(|(&prev, &next)| next > prev)
        .count()
}
