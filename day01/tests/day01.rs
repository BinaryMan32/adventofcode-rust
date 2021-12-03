#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use adventofcode_rust::day01::{part1, part2};

    fn read_lines<P>(filename: P) -> Vec<i64>
    where P: AsRef<Path> {
        if let Ok(file) = File::open(filename) {
            io::BufReader::new(file)
                .lines()
                .into_iter()
                .map(|line| match line {
                    Ok(ln) => ln.parse::<i64>().unwrap(),
                    Err(_) => 0
                })
                .collect()
        } else {
            Vec::<i64>::new()
        }
    }

    #[test]
    fn part1_example() {
        let example: Vec<i64> = read_lines("./example");
        assert_eq!(part1(&example), 7);
    }

    #[test]
    fn part1_input() {
        let input: Vec<i64> = read_lines("./input");
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn part2_example() {
        let example: Vec<i64> = read_lines("./example");
        assert_eq!(part2(&example), 5);
    }

    #[test]
    fn part2_input() {
        let input: Vec<i64> = read_lines("./input");
        assert_eq!(part2(&input), 0);
    }
}
