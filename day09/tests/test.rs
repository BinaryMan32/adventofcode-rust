#[cfg(test)]
mod tests {
    use day09::{part1, part2};

    static EXAMPLE: &str = include_str!("example");
    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE.lines().collect()), 15);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&INPUT.lines().collect()), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EXAMPLE.lines().collect()), 1134);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&INPUT.lines().collect()), 0);
    }
}
