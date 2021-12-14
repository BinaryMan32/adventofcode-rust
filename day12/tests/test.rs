#[cfg(test)]
mod tests {
    use day12::{part1, part2};

    static EXAMPLE1: &str = include_str!("example1");
    static EXAMPLE2: &str = include_str!("example2");
    static EXAMPLE3: &str = include_str!("example3");
    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&EXAMPLE1.lines().collect()), 10);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&EXAMPLE2.lines().collect()), 19);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&EXAMPLE3.lines().collect()), 226);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&INPUT.lines().collect()), 0);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&EXAMPLE1.lines().collect()), 0);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&EXAMPLE2.lines().collect()), 0);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&EXAMPLE3.lines().collect()), 0);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&INPUT.lines().collect()), 0);
    }
}
