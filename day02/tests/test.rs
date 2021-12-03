#[cfg(test)]
mod tests {
    use day02::{part1, part2};

    fn integers(data: &str) -> Vec<i64> {
        data
            .lines()
            .into_iter()
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    }

    static EXAMPLE: &str = include_str!("example");
    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE.lines().collect()), 150);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&INPUT.lines().collect()), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EXAMPLE.lines().collect()), 900);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&INPUT.lines().collect()), 0);
    }
}
